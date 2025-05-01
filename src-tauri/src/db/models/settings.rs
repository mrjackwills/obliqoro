use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::{app_error::AppError, request_handlers::FrontEndState};

const ONE_MINUTE_AS_SEC: u16 = 60;

#[derive(FromRow, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ModelSettings {
    pub auto_pause_threshold: u8,
    pub auto_pause_timespan_sec: u16,
    pub auto_pause: bool,
    pub auto_resume_threshold: u8,
    pub auto_resume_timespan_sec: u16,
    pub auto_resume: bool,
    pub fullscreen: bool,
    pub long_break_as_sec: u16,
    pub number_session_before_break: u8,
    pub session_as_sec: u16,
    pub short_break_as_sec: u16,
}

impl From<&FrontEndState> for ModelSettings {
    fn from(value: &FrontEndState) -> Self {
        Self {
            auto_pause_threshold: value.auto_pause_threshold,
            auto_pause_timespan_sec: value.auto_pause_timespan_sec,
            auto_pause: value.auto_pause,
            auto_resume_threshold: value.auto_resume_threshold,
            auto_resume_timespan_sec: value.auto_resume_timespan_sec,
            auto_resume: value.auto_resume,
            fullscreen: value.fullscreen,
            long_break_as_sec: value.long_break_as_sec,
            number_session_before_break: value.number_session_before_break,
            session_as_sec: value.session_as_sec,
            short_break_as_sec: value.short_break_as_sec,
        }
    }
}

impl ModelSettings {
    const fn default() -> Self {
        Self {
            auto_pause_threshold: 5,
            auto_pause_timespan_sec: 300,
            auto_pause: false,
            auto_resume_threshold: 5,
            auto_resume_timespan_sec: 300,
            auto_resume: false,
            fullscreen: false,
            long_break_as_sec: ONE_MINUTE_AS_SEC * 5,
            number_session_before_break: 4,
            session_as_sec: ONE_MINUTE_AS_SEC * 25,
            short_break_as_sec: ONE_MINUTE_AS_SEC,
        }
    }

    pub async fn update(sqlite: &SqlitePool, settings: &Self) -> Result<(), AppError> {
        let query = "
UPDATE
    settings
SET
    auto_pause_threshold = $1,
    auto_pause_timespan_sec = $2,
    auto_pause = $3,
    auto_resume = $4,
    auto_resume_threshold = $5,
    auto_resume_timespan_sec = $6,
    fullscreen = $7,
    long_break_as_sec = $8,
    number_session_before_break = $9,
    session_as_sec = $10,
    short_break_as_sec = $11";
        sqlx::query(query)
            .bind(settings.auto_pause_threshold)
            .bind(settings.auto_pause_timespan_sec)
            .bind(settings.auto_pause)
            .bind(settings.auto_resume)
            .bind(settings.auto_resume_threshold)
            .bind(settings.auto_resume_timespan_sec)
            .bind(settings.fullscreen)
            .bind(settings.long_break_as_sec)
            .bind(settings.number_session_before_break)
            .bind(settings.session_as_sec)
            .bind(settings.short_break_as_sec)
            .execute(sqlite)
            .await?;
        Ok(())
    }

    /// Check if has any settings in database, and insert default if none found
    pub async fn init(sqlite: &SqlitePool) -> Result<Self, AppError> {
        if let Some(settings) = Self::get(sqlite).await? {
            return Ok(settings);
        }
        Self::set_default(sqlite).await
    }

    /// Insert settings row in database, this WILL crash if a settings row is already in the database, as is limited by settings_id = 1!
    async fn set_default(sqlite: &SqlitePool) -> Result<Self, AppError> {
        let settings = Self::default();
        let query = "
INSERT INTO 
   settings(
    auto_pause_threshold,
    auto_pause_timespan_sec,
    auto_pause,
    auto_resume,
    auto_resume_threshold,
    auto_resume_timespan_sec,
    fullscreen,
    long_break_as_sec,
    number_session_before_break,
    session_as_sec,
    short_break_as_sec
    )
VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
        sqlx::query(query)
            .bind(settings.auto_pause_threshold)
            .bind(settings.auto_pause_timespan_sec)
            .bind(settings.auto_pause)
            .bind(settings.auto_resume)
            .bind(settings.auto_resume_threshold)
            .bind(settings.auto_resume_timespan_sec)
            .bind(settings.fullscreen)
            .bind(settings.long_break_as_sec)
            .bind(settings.number_session_before_break)
            .bind(settings.session_as_sec)
            .bind(settings.short_break_as_sec)
            .execute(sqlite)
            .await?;
        Ok(settings)
    }

    /// Update the settings to a default state
    pub async fn reset_settings(sqlite: &SqlitePool) -> Result<Self, AppError> {
        let settings = Self::default();
        Self::update(sqlite, &settings).await?;
        Ok(settings)
    }

    /// Return an optional Settings struct, to check whether need to insert one or not
    async fn get(sqlite: &SqlitePool) -> Result<Option<Self>, AppError> {
        let query = "SELECT * FROM settings";
        Ok(sqlx::query_as::<_, Self>(query)
            .fetch_optional(sqlite)
            .await?)
    }
}
