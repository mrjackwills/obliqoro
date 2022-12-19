use std::{fmt, path::PathBuf, time::Instant};

use rand::seq::SliceRandom;
use sqlx::{Pool, Sqlite};
use tokio::{sync::broadcast::Sender, task::JoinHandle};

use crate::{
    app_error::AppError,
    db::{self, ModelSettings},
    internal_message_handler::InternalMessage,
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

#[derive(Debug)]
pub struct ApplicationState {
    pub session_status: SessionStatus,
    pub sqlite: Pool<Sqlite>,
    pub sx: Sender<InternalMessage>,
    pub tick_process: Option<JoinHandle<()>>,
    // TODO button on frontend to open this location
    _data_location: PathBuf,
    timer_start: Instant,
    paused: bool,
    session_count: u8,
    settings: ModelSettings,
    strategies: Vec<String>,
}

impl ApplicationState {
    pub async fn new(
        app_dir: Option<PathBuf>,
        sx: &Sender<InternalMessage>,
    ) -> Result<Self, AppError> {
        if let Some(local_dir) = app_dir {
            if std::fs::metadata(&local_dir).is_err() && std::fs::create_dir(&local_dir).is_err() {
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
                _data_location: local_dir,
                session_count: 0,
                strategies,
                timer_start: std::time::Instant::now(),
                paused: false,
                session_status: SessionStatus::Work,
                settings,
                sqlite,
                sx: sx.clone(),
                tick_process: None,
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
        if sec < 60 {
            String::from("less than 1 minute")
        } else {
            let minutes = (f64::try_from(sec).unwrap_or(0.0) / 60.0).round();
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

    // Return, in seconds, the current amount left of the onoing work - or break - session
    pub fn current_timer_left(&self) -> u16 {
        let taken_since = u16::try_from(
            std::time::Instant::now()
                .duration_since(self.timer_start)
                .as_secs(),
        )
        .unwrap_or(0);
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
        self.paused = !self.paused;
    }

    /// Check if the timer (tick process) is paused
    pub const fn get_paused(&self) -> bool {
        self.paused
    }

    pub fn reset_timer(&mut self) {
        self.timer_start = std::time::Instant::now();
    }
    /// Start the timer, by saetting the next_break_in value
    pub fn start_work_session(&mut self) {
        self.session_status = SessionStatus::Work;
        self.reset_timer();
    }

    /// Start the break session
    /// TODO update db session count
    pub fn start_break_session(&mut self) {
        let break_type = if self.session_count < self.settings.number_session_before_break {
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

    /// Create a string `next long break after x session[s]`, for frontend and systemtray
    pub fn get_sessions_before_long_title(&self) -> String {
        let number_before_long = self.get_session_before_long_break();
        let mut title = String::from("next long break after ");
        match number_before_long {
            2.. => title.push_str(&format!("{number_before_long} sessions")),
            1 => title.push_str(&format!("{number_before_long} session")),
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

    /// Set the in memory settings to a new ModelSettings objects, is written to sql seperately
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

    // /// close the sql connection in a tokio thead
    // /// Honestly think this is pointless
    // pub fn close_sql(&mut self) {
    //     let sql = self.sqlite.clone();
    //     tokio::spawn(async move {
    //         sql.close().await;
    //     });
    // }
}
