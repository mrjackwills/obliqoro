#![allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::{types::time::PrimitiveDateTime, FromRow, Pool, Sqlite};

use crate::app_error::AppError;

/// This is on the TODO list

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct ModelStats {
    pub date: PrimitiveDateTime,
    pub number_sessions_completed: i64,
}

impl ModelStats {
    pub async fn get(sqlite: &Pool<Sqlite>) -> Result<Vec<Self>, AppError> {
        let query = "SELECT * FROM stats";
        Ok(sqlx::query_as::<_, Self>(query).fetch_all(sqlite).await?)
    }
}
