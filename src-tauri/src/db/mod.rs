use sqlx::{sqlite::SqliteJournalMode, ConnectOptions, SqlitePool};
use std::path::PathBuf;

mod models;

pub use models::settings::ModelSettings;

use crate::app_error::AppError;
/// Open Sqlite pool connection, and return
/// `max_connections` need to be 1, [see issue](https://github.com/launchbadge/sqlx/issues/816)
async fn get_db(path: &PathBuf) -> Result<SqlitePool, sqlx::Error> {
    let mut connect_options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);
    connect_options = connect_options.disable_statement_logging();
    let db = sqlx::pool::PoolOptions::<sqlx::Sqlite>::new()
        .max_connections(1)
        .connect_with(connect_options)
        .await?;
    Ok(db)
}

async fn run_migrations(db: &SqlitePool) {
    let migrations = include_str!("migrations.sql");
    if let Err(e) = sqlx::query(migrations).execute(db).await {
        println!("{e:?}");
        // TODO - handle this better
        std::process::exit(1);
    }
}

/// Create, if they don't exists, all the sql tables
async fn create_tables(db: &SqlitePool) {
    let init_db = include_str!("init_db.sql");
    if let Err(e) = sqlx::query(init_db).execute(db).await {
        println!("{e:?}");
        // TODO - handle this better
        std::process::exit(1);
    }
}

/// Init db connection, works if folder/files exists or not
pub async fn init_db(path: &PathBuf) -> Result<SqlitePool, AppError> {
    let db = get_db(path).await?;
    create_tables(&db).await;
    run_migrations(&db).await;
    Ok(db)
}
