use auto_launch::AutoLaunch;
use rand::seq::IndexedRandom;
use sqlx::SqlitePool;
use std::{
    collections::VecDeque,
    fmt::{self, Write},
    path::PathBuf,
    sync::LazyLock,
    time::Instant,
};
use tokio::{sync::broadcast::Sender, task::JoinHandle};

use crate::{
    app_error::AppError,
    backend_message_handler::{BreakMessage, InternalMessage},
    check_version,
    db::{self, ModelSettings},
    request_handlers::{CpuMeasure, FrontEndState, ToFrontEnd},
};

use tracing::Level;
use tracing_subscriber::{fmt as t_fmt, prelude::__tracing_subscriber_SubscriberExt};

const ONE_WEEK_AS_SEC: u64 = 60 * 60 * 24 * 7;

/// Setup tracing - warning this can write huge amounts to disk
#[cfg(debug_assertions)]
fn setup_tracing(app_dir: &PathBuf) -> Result<(), AppError> {
    let logfile = tracing_appender::rolling::never(app_dir, "obliqoro.log");

    let log_fmt = t_fmt::Layer::default()
        .json()
        .flatten_event(true)
        .with_writer(logfile);

    match tracing::subscriber::set_global_default(
        t_fmt::Subscriber::builder()
            .with_file(true)
            .with_line_number(true)
            .with_max_level(Level::DEBUG)
            .finish()
            .with(log_fmt),
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Internal("Unable to start tracing".to_owned()))
        }
    }
}

/// Setup tracing - warning this can write huge amounts to disk
#[cfg(not(debug_assertions))]
fn setup_tracing(_app_dir: &PathBuf) -> Result<(), AppError> {
    let level = Level::INFO;
    let log_fmt = t_fmt::Layer::default().json().flatten_event(true);

    match tracing::subscriber::set_global_default(
        t_fmt::Subscriber::builder()
            .with_file(false)
            .with_line_number(true)
            .with_max_level(level)
            .finish()
            .with(log_fmt),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Internal("Unable to start tracing".to_owned()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum BreakVariant {
    Short,
    Long,
}

impl fmt::Display for BreakVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Load the Oblique Stratergies into a Lazylock vec
pub static STRATEGIES: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("../oblique.txt")
        .to_owned()
        .lines()
        .map(std::borrow::ToOwned::to_owned)
        .collect::<Vec<_>>()
});

/// Store a most 15 minutes worth of cpu data in the vecdeque
const CPU_VECDEQUE_LEN: usize = 60 * 15;

// #[derive(Debug)]
pub struct ApplicationState {
    pub heartbeat_process: Option<JoinHandle<()>>,
    pub pause_after_break: bool,
    pub session_status: SessionStatus,
    pub sqlite: SqlitePool,
    pub sx: Sender<InternalMessage>,
    pub system_tray_menu: tauri::menu::Menu<tauri::Wry>,
    start_time: std::time::Instant,
    cpu_usage: VecDeque<f32>,
    data_location: PathBuf,
    session_count: u8,
    settings: ModelSettings,
    timer: Timer,
}

impl ApplicationState {
    pub async fn new(
        data_location: PathBuf,
        sx: Sender<InternalMessage>,
        system_tray_menu: tauri::menu::Menu<tauri::Wry>,
    ) -> Result<Self, AppError> {
        let err = || {
            Err(AppError::FS(
                "Can't read or write to app data location".to_owned(),
            ))
        };

        if !std::fs::exists(&data_location).unwrap_or_default()
            && std::fs::create_dir(&data_location).is_err()
        {
            return err();
        }
        setup_tracing(&data_location)?;

        let sqlite = db::init_db(&data_location).await?;
        let settings = ModelSettings::init(&sqlite).await?;

        Ok(Self {
            cpu_usage: VecDeque::with_capacity(CPU_VECDEQUE_LEN),
            data_location,
            heartbeat_process: None,
            start_time: std::time::Instant::now(),
            pause_after_break: false,
            session_count: 0,
            session_status: SessionStatus::Work,
            settings,
            sqlite,
            sx,
            system_tray_menu,
            timer: Timer::default(),
        })
    }

    /// Check if current on a break
    pub fn on_break(&self) -> bool {
        self.session_status != SessionStatus::Work
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

    /// Return a random Oblique strategy
    fn random_strategy() -> String {
        STRATEGIES
            .choose(&mut rand::rng())
            .map_or(String::new(), std::clone::Clone::clone)
    }

    /// Get the settings for starting a break
    pub fn get_break_settings(&self) -> (u16, String) {
        (self.current_timer_left(), Self::random_strategy())
    }

    /// Get the directory where the database is stored
    pub const fn get_data_location(&self) -> &PathBuf {
        &self.data_location
    }

    /// Check if have been running for more than one week,
    /// If so check for updates & reset start timer
    pub fn update_timer_check(&mut self) {
        if self.start_time.elapsed().as_secs() >= ONE_WEEK_AS_SEC {
            self.start_time = std::time::Instant::now();
            check_version::fetch_updates(self.sx.clone());
        }
    }

    /// Return, in seconds, the current amount left of the onoing work - or break - session
    pub fn current_timer_left(&self) -> u16 {
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

    /// Toggle the pause status & return the pause status
    pub fn toggle_pause(&mut self) -> bool {
        self.timer = self.timer.toggle();
        self.get_paused()
    }

    /// Check if the timer (heartbeat_process) is paused
    pub const fn get_paused(&self) -> bool {
        matches!(self.timer, Timer::Paused(_))
    }

    pub fn reset_timer(&mut self) {
        self.timer = self.timer.reset();
    }

    /// Start the timer, by saetting the next_break_in value
    pub fn start_work_session(&mut self) {
        self.session_status = SessionStatus::Work;
        self.reset_timer();
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

    /// Create a string `next break in x`, for frontend and systemtray
    pub fn get_next_break_title(&self) -> String {
        format!(
            "next break in {}",
            Self::format_sec_to_min(self.current_timer_left())
        )
    }

    /// Attempt to get an AutoLaunch using name and path
    fn auto_launch() -> Option<AutoLaunch> {
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

    /// Create a FrontEndState object from the ApplicationState
    pub fn get_state(&self) -> FrontEndState {
        FrontEndState {
            auto_pause_threshold: self.settings.auto_pause_threshold,
            auto_pause_timespan_sec: self.settings.auto_pause_timespan_sec,
            auto_pause: self.settings.auto_pause,
            auto_resume_threshold: self.settings.auto_resume_threshold,
            auto_resume_timespan_sec: self.settings.auto_resume_timespan_sec,
            auto_resume: self.settings.auto_resume,
            fullscreen: self.settings.fullscreen,
            long_break_as_sec: self.settings.long_break_as_sec,
            number_session_before_break: self.settings.number_session_before_break,
            paused: self.get_paused(),
            session_as_sec: self.settings.session_as_sec,
            short_break_as_sec: self.settings.short_break_as_sec,
            start_on_boot: Self::auto_launch().is_some_and(|i| i.is_enabled().unwrap_or_default()),
        }
    }

    /// Update all the settings
    /// Check if session length has changed, and reset timer if so
    pub fn update_all_settings(&mut self, frontend_state: &FrontEndState) {
        if frontend_state.start_on_boot {
            Self::auto_launch().and_then(|i| i.enable().ok());
        } else {
            Self::auto_launch().and_then(|i| i.disable().ok());
        }
        if frontend_state.session_as_sec != self.settings.session_as_sec {
            self.sx.send(InternalMessage::ResetTimer).ok();
        }
        self.settings = ModelSettings::from(frontend_state);
    }

    /// Reset settings to default state
    pub fn set_settings(&mut self, settings: ModelSettings) {
        self.settings = settings;
        Self::auto_launch().and_then(|i| i.disable().ok());
    }

    /// Get fullscreen setting value
    pub const fn get_fullscreen(&self) -> bool {
        self.settings.fullscreen
    }

    /// Calculate the average cpu usage over the previous `limit` seconds
    pub fn calc_cpu_average(&self, limit: u16) -> Option<f32> {
        if self.cpu_usage.len() < limit.into() {
            return None;
        }
        Some(self.cpu_usage.iter().take(limit.into()).sum::<f32>() / f32::from(limit))
    }

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
                    if let Some(avg) = cpu_mesasure.resume {
                        if avg >= f32::from(self.settings.auto_resume_threshold) {
                            self.sx.send(InternalMessage::Pause).ok();
                            self.sx
                                .send(InternalMessage::ToFrontEnd(ToFrontEnd::GetSettings))
                                .ok();
                        }
                    }
                } else if !is_paused && self.settings.auto_pause {
                    if let Some(avg) = cpu_mesasure.pause {
                        if avg <= f32::from(self.settings.auto_pause_threshold) {
                            self.sx.send(InternalMessage::Pause).ok();
                            self.sx
                                .send(InternalMessage::ToFrontEnd(ToFrontEnd::GetSettings))
                                .ok();
                        }
                    }
                }
            }
            self.sx
                .send(InternalMessage::ToFrontEnd(ToFrontEnd::Cpu(cpu_mesasure)))
                .ok();
        }
    }

    /// Auto Pause/Resume, send timer stats
    pub fn on_heartbeat(&mut self, cpu_usage: Option<f32>) {
        self.handle_auto_pause_resume(cpu_usage);

        if !self.get_paused() {
            match self.session_status {
                SessionStatus::Break(_) => {
                    self.sx
                        .send(InternalMessage::ToFrontEnd(ToFrontEnd::OnBreak))
                        .ok();
                    if self.current_timer_left() < 1 {
                        self.sx.send(InternalMessage::Break(BreakMessage::End)).ok();
                    }
                }
                SessionStatus::Work => {
                    self.sx.send(InternalMessage::UpdateMenuTimer).ok();
                    if self.current_timer_left() < 1 {
                        self.sx
                            .send(InternalMessage::Break(BreakMessage::Start))
                            .ok();
                    }
                }
            }
        }
    }
}
