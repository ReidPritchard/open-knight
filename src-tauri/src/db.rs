use std::time::Duration;

use log::error;
use sea_orm::*;
use sea_orm_migration::*;

use crate::{migrations::Migrator, models, utils::AppError};

// Constants
const DATABASE_URL: &str = "sqlite://chess.db?mode=rwc";

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
    Migrator::down(db, None)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to reset database: {}", e)))?;

    // delay for 1 second
    tokio::time::sleep(Duration::from_secs(1)).await;

    Migrator::up(db, None).await.map_err(|e| {
        AppError::DatabaseError(format!("Failed to run migrations after reset: {}", e))
    })?;
    Ok(())
}

pub async fn load_pgn_file(
    db: &DatabaseConnection,
    file_path: &str,
) -> Result<Vec<models::ChessGame>, AppError> {
    let pgn_content =
        std::fs::read_to_string(file_path).map_err(|e| AppError::IoError(e.to_string()))?;
    models::ChessGame::save_from_pgn(db, &pgn_content).await
}

#[derive(Debug, Clone, Default)]
pub struct DatabaseConfig {
    pub url: Option<String>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
}

pub async fn connect_db(config: Option<DatabaseConfig>) -> Result<DatabaseConnection, AppError> {
    let config = config.unwrap_or_default();

    let mut options = ConnectOptions::new(config.url.unwrap_or(DATABASE_URL.to_string()));
    options.max_connections(config.max_connections.unwrap_or(5));
    options.min_connections(config.min_connections.unwrap_or(1));

    Database::connect(options)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to connect to database: {}", e)))
}

// Re-export commonly used types
pub use sea_orm::{DatabaseConnection, DbErr};
