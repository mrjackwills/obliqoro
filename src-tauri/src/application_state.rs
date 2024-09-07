use std::{fmt, path::PathBuf, time::Instant};

use rand::seq::SliceRandom;
use sqlx::SqlitePool;
use tokio::{sync::broadcast::Sender, task::JoinHandle};

use crate::{
    app_error::AppError,
    db::{self, ModelSettings},
    internal_message_handler::{BreakMessage, Emitter, InternalMessage},
    setup_tracing,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Break {
    Short,
    Long,
}

impl fmt::Display for Break {
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
    Break(Break),
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

#[derive(Debug)]
pub struct ApplicationState {
    pub session_status: SessionStatus,
    pub sqlite: SqlitePool,
    pub sx: Sender<InternalMessage>,
    pub tick_process: Option<JoinHandle<()>>,
    pub pause_after_break: bool,
    // TODO button on frontend to open this location?
    data_location: PathBuf,
    session_count: u8,
    settings: ModelSettings,
    strategies: Vec<String>,
    timer: Timer,
}

impl ApplicationState {
    pub async fn new(
        app_dir: Option<PathBuf>,
        sx: &Sender<InternalMessage>,
    ) -> Result<Self, AppError> {
        if let Some(local_dir) = app_dir {
            if !std::fs::exists(&local_dir).unwrap_or_default()
                && std::fs::create_dir(&local_dir).is_err()
            {
                return Err(AppError::FS("Can't read or write app data".to_owned()));
            }
            setup_tracing(&local_dir)?;
            let db_location = PathBuf::from(format!("{}/obliqoro.db", local_dir.display()));
            let sqlite = db::init_db(&db_location).await?;
            let settings = ModelSettings::init(&sqlite).await?;
            let strategies = include_str!("../oblique.txt")
                .to_owned()
                .lines()
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<_>>();
            Ok(Self {
                data_location: local_dir,
                pause_after_break: false,
                session_count: 0,
                session_status: SessionStatus::Work,
                settings,
                sqlite,
                strategies,
                sx: sx.clone(),
                tick_process: None,
                timer: Timer::default(),
            })
        } else {
            Err(AppError::FS("Can't read or write app data".to_owned()))
        }
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
    fn random_strategy(&self) -> String {
        let mut rng = rand::thread_rng();
        self.strategies
            .choose(&mut rng)
            .map_or(String::new(), std::clone::Clone::clone)
    }

    /// Get the settings for starting a break
    pub fn get_break_settings(&self) -> (u16, String) {
        (self.current_timer_left(), self.random_strategy())
    }

    /// Get the directory where the database is stored
    pub const fn get_data_location(&self) -> &PathBuf {
        &self.data_location
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
                Break::Short => {
                    u16::from(self.settings.short_break_as_sec).saturating_sub(taken_since)
                }
                Break::Long => self.settings.long_break_as_sec.saturating_sub(taken_since),
            },
            SessionStatus::Work => self.settings.session_as_sec.saturating_sub(taken_since),
        }
    }

    /// Toggle the pause status
    pub fn toggle_pause(&mut self) {
        self.timer = self.timer.toggle();
    }

    /// Check if the timer (tick process) is paused
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
            Break::Short
        } else {
            self.session_count = 0;
            Break::Long
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
            2.. => title.push_str(&format!("{number_before_long} sessions")),
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

    /// Return ModelSettings object
    pub const fn get_settings(&self) -> ModelSettings {
        self.settings
    }

    /// Set the in memory settings to a new ModelSettings objects, is written to sql separately
    pub fn reset_settings(&mut self, settings: ModelSettings) {
        self.settings = settings;
    }

    /// Get fullscreen setting value
    pub const fn get_fullscreen(&self) -> bool {
        self.settings.fullscreen
    }

    /// Set fullscreen setting value
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.settings.fullscreen = fullscreen;
    }

    /// Get long_break setting value
    pub fn set_long_break_as_sec(&mut self, i: u16) {
        self.settings.long_break_as_sec = i;
    }

    /// Set long_break setting value
    pub fn set_number_session_before_break(&mut self, i: u8) {
        self.settings.number_session_before_break = i;
    }

    /// Set session setting value
    pub fn set_session_as_sec(&mut self, i: u16) {
        self.settings.session_as_sec = i;
    }

    /// Set short_break setting value
    pub fn set_short_break_as_sec(&mut self, i: u8) {
        self.settings.short_break_as_sec = i;
    }

    pub fn tick_process(&self) {
        if !self.get_paused() {
            match self.session_status {
                SessionStatus::Break(_) => {
                    self.sx.send(InternalMessage::Emit(Emitter::OnBreak)).ok();
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
    // /// close the sql connection in a tokio thead
    // /// Honestly think this is pointless
    // pub fn close_sql(&mut self) {
    //     let sql = self.sqlite.clone();
    //     tokio::spawn(async move {
    //         sql.close().await;
    //     });
    // }
}
