use std::{
    collections::VecDeque,
    fmt::Write,
    path::PathBuf,
    sync::{Arc, LazyLock},
    time::Instant,
};

use auto_launch::AutoLaunch;
use rand::seq::IndexedRandom;
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter, Wry, menu::MenuItemKind};
use tokio::{sync::broadcast::Sender, task::JoinHandle};

use crate::{
    MAIN_WINDOW,
    app_error::AppError,
    application_state::{
        menu::MenuManipulation,
        system_tray::{MenuEntry, change_menu_entry_status, set_icon},
        window_action::WindowAction,
    },
    check_version,
    db::ModelSettings,
    message_handler::{MsgB, MsgFE, MsgI, MsgWV},
    request_handlers::{CpuMeasure, FrontEndState, ShowTimer},
};

mod menu;
mod system_tray;
mod window_action;

pub use system_tray::create_system_tray;

/// Store a most 15 minutes worth of cpu data in the vecdeque
const CPU_VECDEQUE_LEN: usize = 60 * 15;

const ONE_WEEK_AS_SEC: u64 = 60 * 60 * 24 * 7;

/// Load the Oblique Stratergies into a Lazylock vec
pub static STRATEGIES: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("../../oblique.txt")
        .to_owned()
        .lines()
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<_>>()
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum BreakVariant {
    Short,
    Long,
}

impl std::fmt::Display for BreakVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "short"),
            Self::Long => write!(f, "long"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum SessionStatus {
    Work,
    Break(BreakVariant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Timer {
    Paused((Instant, Instant)),
    Work(Instant),
}
impl Default for Timer {
    fn default() -> Self {
        Self::Work(std::time::Instant::now())
    }
}

impl Timer {
    fn toggle(self) -> Self {
        match self {
            Self::Paused((original, paused)) => Self::Work(original + paused.elapsed()),
            Self::Work(original) => Self::Paused((original, std::time::Instant::now())),
        }
    }
    fn reset(self) -> Self {
        match self {
            Self::Work(_) => Self::default(),
            Self::Paused(_) => Self::Paused((std::time::Instant::now(), std::time::Instant::now())),
        }
    }
}

impl From<&ApplicationState> for FrontEndState {
    fn from(state: &ApplicationState) -> Self {
        Self {
            auto_pause_threshold: state.settings.auto_pause_threshold,
            auto_pause_timespan_sec: state.settings.auto_pause_timespan_sec,
            auto_pause: state.settings.auto_pause,
            auto_resume_threshold: state.settings.auto_resume_threshold,
            auto_resume_timespan_sec: state.settings.auto_resume_timespan_sec,
            auto_resume: state.settings.auto_resume,
            fullscreen: state.settings.fullscreen,
            long_break_as_sec: state.settings.long_break_as_sec,
            number_session_before_break: state.settings.number_session_before_break,
            paused: state.get_paused(),
            session_as_sec: state.settings.session_as_sec,
            short_break_as_sec: state.settings.short_break_as_sec,
            start_on_boot: ApplicationState::get_auto_launch()
                .is_some_and(|i| i.is_enabled().unwrap_or_default()),
        }
    }
}

/// fuzzy second to minutes conversion
fn format_sec_to_min(sec: u16) -> String {
    if sec <= 60 {
        String::from("less than 1 minute")
    } else {
        let minutes = (f64::from(sec) / 60.0).ceil();
        format!("{minutes} minutes")
    }
}

pub struct ApplicationState {
    app_handle: AppHandle,
    cpu_usage: VecDeque<f32>,
    data_location: PathBuf,
    heartbeat_process: Option<Arc<JoinHandle<()>>>,
    pause_after_break: bool,
    session_count: u8,
    session_status: SessionStatus,
    settings: ModelSettings,
    sqlite: SqlitePool,
    start_time: std::time::Instant,
    sx: Sender<MsgI>,
    system_tray_menu: tauri::menu::Menu<tauri::Wry>,
    timer: Timer,
}

impl ApplicationState {
    pub fn new(
        app_handle: AppHandle,
        data_location: PathBuf,
        sx: Sender<MsgI>,
        settings: ModelSettings,
        sqlite: sqlx::Pool<sqlx::Sqlite>,
        system_tray_menu: tauri::menu::Menu<tauri::Wry>,
    ) -> Self {
        Self {
            app_handle,
            cpu_usage: VecDeque::with_capacity(CPU_VECDEQUE_LEN),
            data_location,
            heartbeat_process: None,
            pause_after_break: false,
            session_count: 0,
            session_status: SessionStatus::Work,
            settings,
            sqlite,
            start_time: std::time::Instant::now(),
            sx,
            system_tray_menu,
            timer: Timer::default(),
        }
    }

    /// Calculate the average cpu usage over the previous `limit` seconds
    pub fn calc_cpu_average(&self, limit: u16) -> Option<f32> {
        if self.cpu_usage.len() < limit.into() {
            return None;
        }
        Some(self.cpu_usage.iter().take(limit.into()).sum::<f32>() / f32::from(limit))
    }

    /// Handle all internal messages about emitting messages to the frontend, and send to the frontend
    pub fn emit_to_frontend(&self, msg_to_frontend: MsgFE) {
        let event_name = msg_to_frontend.as_str();
        match msg_to_frontend {
            MsgFE::GoToSettings => {
                let on_break = self.get_on_break();
                if !on_break {
                    self.app_handle
                        .emit_str(MAIN_WINDOW, event_name.to_owned())
                        .ok();
                    WindowAction::toggle_visibility(&self.app_handle, false);
                }
            }
            MsgFE::Cpu(value) => {
                self.app_handle.emit_to(MAIN_WINDOW, event_name, value).ok();
            }

            MsgFE::NextBreak => {
                self.app_handle
                    .emit_to(MAIN_WINDOW, event_name, self.get_next_break_title())
                    .ok();
            }

            MsgFE::OnBreak => {
                self.app_handle
                    .emit_to(MAIN_WINDOW, event_name, self.get_current_timer_left())
                    .ok();
            }

            MsgFE::Error => {
                self.app_handle
                    .emit_to(MAIN_WINDOW, event_name, "Internal Error")
                    .ok();
            }

            MsgFE::GetSettings => {
                self.app_handle
                    .emit_to(MAIN_WINDOW, event_name, self.get_frontend_state())
                    .ok();
            }
            MsgFE::SessionsBeforeLong => {
                self.app_handle
                    .emit_to(
                        MAIN_WINDOW,
                        event_name,
                        self.get_sessions_before_long_title(),
                    )
                    .ok();
            }
            MsgFE::GoToTimer => {
                let (break_time, strategy) = self.get_break_settings();
                self.app_handle
                    .emit_to(MAIN_WINDOW, "fullscreen", self.get_fullscreen())
                    .ok();
                self.app_handle
                    .emit_to(
                        MAIN_WINDOW,
                        event_name,
                        ShowTimer::new(break_time, strategy),
                    )
                    .ok();
            }
            MsgFE::PackageInfo(info) => {
                self.app_handle.emit_to(MAIN_WINDOW, event_name, info).ok();
            }
            MsgFE::Paused(paused) => {
                self.app_handle
                    .emit_to(MAIN_WINDOW, event_name, paused)
                    .ok();
            }
        }
    }

    // Various `get_x` methods

    pub const fn get_app_handle(&self) -> &AppHandle {
        &self.app_handle
    }

    /// Attempt to get an AutoLaunch using name and path
    fn get_auto_launch() -> Option<AutoLaunch> {
        tauri::utils::platform::current_exe().map_or(None, |app_exe| {
            let app_path = dunce::canonicalize(app_exe).unwrap_or_default();
            let app_name = app_path.file_stem().unwrap_or_default().to_os_string();
            Some(AutoLaunch::new(
                app_name.to_str().unwrap_or_default(),
                app_path.to_str().unwrap_or_default(),
                &[] as &[&str],
            ))
        })
    }

    /// Get the settings for starting a break
    pub fn get_break_settings(&self) -> (u16, String) {
        (self.get_current_timer_left(), Self::get_random_strategy())
    }

    /// Return, in seconds, the current amount left of the onoing work - or break - session
    pub fn get_current_timer_left(&self) -> u16 {
        let taken_since = match self.timer {
            Timer::Paused(_) => 0,
            Timer::Work(timer) => {
                u16::try_from(std::time::Instant::now().duration_since(timer).as_secs())
                    .unwrap_or_default()
            }
        };
        match self.session_status {
            SessionStatus::Break(break_type) => match break_type {
                BreakVariant::Short => self.settings.short_break_as_sec.saturating_sub(taken_since),
                BreakVariant::Long => self.settings.long_break_as_sec.saturating_sub(taken_since),
            },
            SessionStatus::Work => self.settings.session_as_sec.saturating_sub(taken_since),
        }
    }

    /// Get the directory where the database is stored
    pub const fn get_data_location(&self) -> &PathBuf {
        &self.data_location
    }

    /// Create a FrontEndState object from the ApplicationState
    pub fn get_frontend_state(&self) -> FrontEndState {
        FrontEndState::from(self)
    }

    /// Get fullscreen setting value
    pub const fn get_fullscreen(&self) -> bool {
        self.settings.fullscreen
    }

    pub fn get_menu_entry(&self, entry: MenuEntry) -> Option<MenuItemKind<Wry>> {
        self.system_tray_menu.get(entry.get_id())
    }

    /// Create a string `next break in x`, for frontend and systemtray
    pub fn get_next_break_title(&self) -> String {
        format!(
            "next break in {}",
            format_sec_to_min(self.get_current_timer_left())
        )
    }

    /// Check if current on a break
    pub fn get_on_break(&self) -> bool {
        self.session_status != SessionStatus::Work
    }

    /// Check if the timer (heartbeat_process) is paused
    pub const fn get_paused(&self) -> bool {
        matches!(self.timer, Timer::Paused(_))
    }

    /// Return a random Oblique strategy
    fn get_random_strategy() -> String {
        STRATEGIES
            .choose(&mut rand::rng())
            .map_or(String::new(), std::clone::Clone::clone)
    }

    /// Return the number of short sessions before the next long break
    const fn get_session_before_long_break(&self) -> u8 {
        self.settings
            .number_session_before_break
            .saturating_sub(self.session_count)
    }

    /// Create a string `next long break after x sessions`, for frontend and systemtray
    pub fn get_sessions_before_long_title(&self) -> String {
        let number_before_long = self.get_session_before_long_break();
        let mut title = String::from("next long break after ");
        match number_before_long {
            2.. => {
                write!(&mut title, "{number_before_long} sessions").ok();
            }
            _ => title.push_str("current session"),
        }
        title
    }

    // Situation handlers

    /// Calculate the current pause & resume averages, apply pause or resume, send details to frontend
    fn handle_auto_pause_resume(&mut self, current_usage: Option<f32>) {
        if let Some(cpu_usage) = current_usage {
            if self.cpu_usage.len() >= CPU_VECDEQUE_LEN {
                self.cpu_usage.pop_back();
            }
            let cpu_mesasure = CpuMeasure {
                current: cpu_usage,
                pause: self.calc_cpu_average(self.settings.auto_pause_timespan_sec),
                resume: self.calc_cpu_average(self.settings.auto_resume_timespan_sec),
            };

            self.cpu_usage.push_front(cpu_usage);

            let is_paused = self.get_paused();

            if self.session_status == SessionStatus::Work {
                if is_paused && self.settings.auto_resume {
                    if let Some(avg) = cpu_mesasure.resume
                        && avg >= f32::from(self.settings.auto_resume_threshold)
                    {
                        self.sx.send(MsgI::Pause).ok();
                        self.sx.send(MsgI::ToFrontEnd(MsgFE::GetSettings)).ok();
                    }
                } else if !is_paused
                    && self.settings.auto_pause
                    && let Some(avg) = cpu_mesasure.pause
                    && avg <= f32::from(self.settings.auto_pause_threshold)
                {
                    self.sx.send(MsgI::Pause).ok();
                    self.sx.send(MsgI::ToFrontEnd(MsgFE::GetSettings)).ok();
                }
            }
            self.sx
                .send(MsgI::ToFrontEnd(MsgFE::Cpu(cpu_mesasure)))
                .ok();
        }
    }

    /// Handle all internal messages about the Break/Session stats
    pub fn handle_break(&mut self, break_message: MsgB) {
        let fullscreen = self.get_fullscreen();
        match break_message {
            MsgB::Start => {
                self.start_break_session();
                change_menu_entry_status(&self.system_tray_menu, false);
                self.sx.send(MsgI::ToFrontEnd(MsgFE::GoToTimer)).ok();
                WindowAction::show_window(&self.app_handle, fullscreen);
            }
            MsgB::End => {
                self.start_work_session();
                change_menu_entry_status(&self.system_tray_menu, true);
                if self.pause_after_break {
                    self.sx.send(MsgI::Pause).ok();
                    // if the app is in fullscreen mode, need to remove the fullscreen, normally this is handled by the hide_window function, but it's not being called here
                    WindowAction::remove_fullscreen(&self.app_handle);
                } else {
                    WindowAction::hide_window(&self.app_handle, fullscreen);
                    MenuManipulation::update_all(self);
                }
                self.pause_after_break = false;
            }
        }
    }

    /// Handle all internal messages about window visibility
    pub fn handle_visibility(&self, window_visibility: MsgWV) {
        let on_break = self.get_on_break();
        match window_visibility {
            MsgWV::Close => {
                if !on_break {
                    self.app_handle.exit(0);
                }
            }
            MsgWV::Hide => {
                if !on_break {
                    WindowAction::hide_window(&self.app_handle, false);
                }
            }
            MsgWV::Minimize => {
                WindowAction::hide_window(&self.app_handle, false);
            }
            MsgWV::Show => {
                WindowAction::show_window(&self.app_handle, false);
            }
            MsgWV::Toggle => {
                if !on_break {
                    WindowAction::toggle_visibility(&self.app_handle, false);
                }
            }
        }
    }

    // Heartbeat methods

    /// Abort heartbeat process, and update with new handle
    pub fn heartbeat_update(&mut self, handle: Arc<JoinHandle<()>>) {
        self.heartbeat_abort();
        self.heartbeat_process = Some(handle);
    }

    /// Abort heartbeat process
    pub fn heartbeat_abort(&self) {
        if let Some(handle) = self.heartbeat_process.as_ref() {
            handle.abort();
        }
    }

    /// Auto Pause/Resume, send timer stats
    pub fn on_heartbeat(&mut self, cpu_usage: Option<f32>) {
        self.handle_auto_pause_resume(cpu_usage);

        if !self.get_paused() {
            match self.session_status {
                SessionStatus::Break(_) => {
                    self.sx.send(MsgI::ToFrontEnd(MsgFE::OnBreak)).ok();
                    if self.get_current_timer_left() < 1 {
                        self.sx.send(MsgI::Break(MsgB::End)).ok();
                    }
                }
                SessionStatus::Work => {
                    self.sx.send(MsgI::UpdateMenuTimer).ok();
                    if self.get_current_timer_left() < 1 {
                        self.sx.send(MsgI::Break(MsgB::Start)).ok();
                    }
                }
            }
        }
    }

    // Reset methods

    /// Reset settings to default in SQLite, send new settings to frontend
    pub async fn reset_settings(&mut self) -> Result<(), AppError> {
        let sqlite = self.sqlite.clone();
        let settings = ModelSettings::reset_settings(&sqlite).await?;
        self.set_settings(settings);
        self.reset_timer();
        self.sx.send(MsgI::ToFrontEnd(MsgFE::GetSettings)).ok();
        self.sx
            .send(MsgI::ToFrontEnd(MsgFE::Paused(self.get_paused())))
            .ok();
        Ok(())
    }

    pub fn reset_timer(&mut self) {
        self.timer = self.timer.reset();
    }

    /// Send an internal message
    pub fn send(&self, msg: MsgI) {
        self.sx.send(msg).ok();
    }

    /// Store settings, disable auto launch
    pub fn set_settings(&mut self, settings: ModelSettings) {
        self.settings = settings;
        Self::get_auto_launch().and_then(|i| i.disable().ok());
    }

    /// Start the break session
    /// TODO update db session count
    pub fn start_break_session(&mut self) {
        let break_type = if self.session_count + 1 < self.settings.number_session_before_break {
            self.session_count += 1;
            BreakVariant::Short
        } else {
            self.session_count = 0;
            BreakVariant::Long
        };
        self.reset_timer();
        self.session_status = SessionStatus::Break(break_type);
    }

    /// Start the timer, by saetting the next_break_in value
    pub fn start_work_session(&mut self) {
        self.session_status = SessionStatus::Work;
        self.reset_timer();
    }

    /// Toggle the pause status & return the pause status
    pub fn toggle_pause(&mut self) -> bool {
        self.timer = self.timer.toggle();
        self.get_paused()
    }

    /// Update the pause_after_break value
    pub const fn update_pause_after_break(&mut self, pause: bool) {
        self.pause_after_break = pause;
    }

    /// Update all the settings
    /// Check if session length has changed, and reset timer if so
    pub fn update_all_settings(&mut self, frontend_state: &FrontEndState) {
        if frontend_state.start_on_boot {
            Self::get_auto_launch().and_then(|i| i.enable().ok());
        } else {
            Self::get_auto_launch().and_then(|i| i.disable().ok());
        }
        if frontend_state.session_as_sec != self.settings.session_as_sec {
            self.sx.send(MsgI::ResetTimer).ok();
        }
        self.settings = ModelSettings::from(frontend_state);
    }

    pub fn update_icon(&self, paused: bool) {
        set_icon(&self.app_handle, paused);
    }

    pub fn update_menu_all(&self) {
        MenuManipulation::update_all(self);
    }

    pub fn update_menu_pause(&self, pause: bool) {
        MenuManipulation::update_pause(self, pause);
    }

    /// Save new settings in SQLite, send new settings to the frontend
    pub async fn update_settings(&mut self, frontend_state: FrontEndState) -> Result<(), AppError> {
        let sqlite = self.sqlite.clone();
        let new_settings = ModelSettings::from(&frontend_state);
        ModelSettings::update(&sqlite, &new_settings).await?;
        self.update_all_settings(&frontend_state);
        Ok(())
    }

    /// Check if have been running for more than one week,
    /// If so check for updates & reset start timer
    pub fn update_timer_check(&mut self) {
        if self.start_time.elapsed().as_secs() >= ONE_WEEK_AS_SEC {
            self.start_time = std::time::Instant::now();
            check_version::fetch_updates(self.sx.clone());
        }
    }
}
