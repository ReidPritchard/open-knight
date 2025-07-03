use std::time::Duration;

use log::error;
use sea_orm::*;
use sea_orm_migration::*;
use tauri::{AppHandle, Manager};
use tokio::fs;

use crate::{
    migrations::Migrator, models, utils::AppError, DATABASE_FILE_NAME,
};

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), AppError> {
    // If it fails, try to reset the database
    if let Err(e) = Migrator::up(db, None).await {
        error!("Failed to run migrations: {}", e);
        tokio::time::sleep(Duration::from_secs(1)).await;
        reset_database(db).await?;
    }

    Ok(())
}

pub async fn reset_database(db: &DatabaseConnection) -> Result<(), AppError> {
    Migrator::down(db, None).await.map_err(|e| {
        AppError::DatabaseError(format!("Failed to reset database: {}", e))
    })?;

    // delay for 1 second
    tokio::time::sleep(Duration::from_secs(1)).await;

    Migrator::up(db, None).await.map_err(|e| {
        AppError::DatabaseError(format!(
            "Failed to run migrations after reset: {}",
            e
        ))
    })?;
    Ok(())
}

pub async fn load_pgn_file(
    db: &DatabaseConnection,
    file_path: &str,
) -> Result<Vec<models::ChessGame>, AppError> {
    let pgn_content = std::fs::read_to_string(file_path)
        .map_err(|e| AppError::IoError(e.to_string()))?;
    models::ChessGame::save_from_pgn(db, &pgn_content).await
}

/// Function to resolve the database path/url
///
/// Parameters:
/// - `app_handle`: The Tauri application handle used to access the app's data directory.
///
/// Returns:
/// A string representing the database URL, which can be a file path or a connection string.
pub async fn resolve_database_url(app_handle: &AppHandle) -> String {
    // Check if an environment variable has been set for the database URL
    let env_db_url = std::env::var("DATABASE_URL").ok();
    if let Some(url) = env_db_url {
        return url;
    }

    // The ENV variable was not set, so we use the default database URL
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get the App Data directory");

    // Create the app directory if it doesn't exist
    fs::create_dir_all(&app_dir).await;

    // Construct the full path to the database file
    let database_file_path = app_dir.join(DATABASE_FILE_NAME);

    // Format the database URL for SQLite
    let database_url =
        format!("sqlite://{}?mode=rwc", database_file_path.display());

    database_url
}

#[derive(Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

pub async fn connect_db(
    config: DatabaseConfig
) -> Result<DatabaseConnection, AppError> {
    let mut options = ConnectOptions::new(config.url);
    options.max_connections(config.max_connections);
    options.min_connections(config.min_connections);

    Database::connect(options).await.map_err(|e| {
        AppError::DatabaseError(format!("Failed to connect to database: {}", e))
    })
}

// Re-export commonly used types
pub use sea_orm::{DatabaseConnection, DbErr};
