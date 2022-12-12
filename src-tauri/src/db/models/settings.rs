use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

use crate::app_error::AppError;

const ONE_MINUTE: i64 = 60;

#[derive(FromRow, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct ModelSettings {
    pub short_break_as_sec: i64,
    pub long_break_as_sec: i64,
    pub session_as_sec: i64,
    pub number_session_before_break: u8,
    pub fullscreen: bool,
}

impl ModelSettings {
    const fn default() -> Self {
        Self {
            short_break_as_sec: 30,
            long_break_as_sec: ONE_MINUTE * 5,
            session_as_sec: ONE_MINUTE * 25,
            number_session_before_break: 4,
            fullscreen: false,
        }
    }

    /// Update the settings
    pub async fn update_fullscreen(
        sqlite: &Pool<Sqlite>,
        fullscreen: bool,
    ) -> Result<(), AppError> {
        let query = "UPDATE Settings SET fullscreen = $1";
        sqlx::query(query).bind(fullscreen).execute(sqlite).await?;
        Ok(())
    }

    /// Update shortbreak setting
    pub async fn update_shortbreak(sqlite: &Pool<Sqlite>, shortbreak: i64) -> Result<(), AppError> {
        let query = "UPDATE Settings SET short_break_as_sec = $1";
        sqlx::query(query).bind(shortbreak).execute(sqlite).await?;
        Ok(())
    }

    /// Update long break setting
    pub async fn update_longbreak(sqlite: &Pool<Sqlite>, longbreak: i64) -> Result<(), AppError> {
        let query = "UPDATE Settings SET long_break_as_sec = $1";
        sqlx::query(query).bind(longbreak).execute(sqlite).await?;
        Ok(())
    }

    /// Update session length
    pub async fn update_session(sqlite: &Pool<Sqlite>, session: i64) -> Result<(), AppError> {
        let query = "UPDATE Settings SET session_as_sec = $1";
        sqlx::query(query).bind(session).execute(sqlite).await?;
        Ok(())
    }

    /// Update number sessions before break
    pub async fn update_number_session_before_break(
        sqlite: &Pool<Sqlite>,
        number_session_before_break: u8,
    ) -> Result<(), AppError> {
        let query = "UPDATE Settings SET number_session_before_break = $1";
        sqlx::query(query)
            .bind(number_session_before_break)
            .execute(sqlite)
            .await?;
        Ok(())
    }

    /// Check if has any settings in database, and insert default if none found
    /// maybe should be a transaction?
    pub async fn init(sqlite: &Pool<Sqlite>) -> Result<Self, AppError> {
        if let Some(settings) = Self::get(sqlite).await? {
            return Ok(settings);
        }
        Self::set_default(sqlite).await
    }

    /// Insert settings row in database, this WILL crash if a settings row is already in the database, as is limited by settings_id = 1!
    async fn set_default(sqlite: &Pool<Sqlite>) -> Result<Self, AppError> {
        let settings = Self::default();
        let query = "INSERT INTO Settings(short_break_as_sec, long_break_as_sec, session_as_sec, number_session_before_break, fullscreen) VALUES($1, $2, $3, $4, $5)";
        sqlx::query(query)
            .bind(settings.short_break_as_sec)
            .bind(settings.long_break_as_sec)
            .bind(settings.session_as_sec)
            .bind(settings.number_session_before_break)
            .bind(settings.fullscreen)
            .execute(sqlite)
            .await?;
        Ok(settings)
    }

    /// Update the settings to a default state
    pub async fn reset_settings(sqlite: &Pool<Sqlite>) -> Result<Self, AppError> {
        let settings = Self::default();
        let query = "UPDATE Settings SET short_break_as_sec = $1, long_break_as_sec = $2, session_as_sec = $3, number_session_before_break = $4, fullscreen = $5";
        sqlx::query(query)
            .bind(settings.short_break_as_sec)
            .bind(settings.long_break_as_sec)
            .bind(settings.session_as_sec)
            .bind(settings.number_session_before_break)
            .bind(settings.fullscreen)
            .execute(sqlite)
            .await?;
        Ok(settings)
    }

    /// Return an optional Settings struct, to check whether need to insert one or not
    async fn get(sqlite: &Pool<Sqlite>) -> Result<Option<Self>, AppError> {
        let query = "SELECT * FROM settings";
        Ok(sqlx::query_as::<_, Self>(query)
            .fetch_optional(sqlite)
            .await?)
    }
}
