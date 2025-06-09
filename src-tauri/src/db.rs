use std::time::Duration;

use sea_orm::*;
use sea_orm_migration::*;

use crate::{migrations::Migrator, models, utils::AppError};

// Constants
const DATABASE_URL: &str = "sqlite://chess.db?mode=rwc";

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), AppError> {
    Migrator::up(db, Some(2))
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to run migrations: {}", e)))?;

    Migrator::up(db, None)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to run migrations: {}", e)))?;

    Ok(())
}

pub async fn reset_database(db: &DatabaseConnection) -> Result<(), AppError> {
    Migrator::down(db, None)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to reset database: {}", e)))?;
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

pub async fn connect_db() -> Result<DatabaseConnection, AppError> {
    let mut options = ConnectOptions::new(DATABASE_URL);
    options.max_connections(20); // Increased for better concurrency handling
    options.min_connections(5); // Higher minimum to avoid connection establishment delays
    options.idle_timeout(Duration::from_secs(60)); // Longer idle timeout
    options.acquire_timeout(Duration::from_secs(10)); // Add explicit acquire timeout
    options.connect_timeout(Duration::from_secs(10)); // Add explicit connect timeout

    Database::connect(options)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to connect to database: {}", e)))
}

// Re-export commonly used types
pub use sea_orm::{DatabaseConnection, DbErr};
