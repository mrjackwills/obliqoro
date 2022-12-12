use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("internal error:")]
    Internal(String),
    #[error("FS error:")]
    FS(String),
    #[error("not found")]
    SqlxError(#[from] sqlx::Error),
}
