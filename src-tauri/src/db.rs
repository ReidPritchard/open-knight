use sea_orm::*;
use sea_orm_migration::*;

use crate::{migrations::Migrator, models, utils::AppError};

// Constants
const DATABASE_URL: &str = "sqlite://chess.db?mode=rwc";

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), AppError> {
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
    let pgn_content = std::fs::read_to_string(file_path).map_err(|e| {
        AppError::ParseError(format!("Failed to read PGN file '{}': {}", file_path, e))
    })?;
    models::ChessGame::save_from_pgn(db, &pgn_content).await
}

pub async fn connect_db() -> Result<DatabaseConnection, AppError> {
    Database::connect(DATABASE_URL)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to connect to database: {}", e)))
}

// Re-export commonly used types
pub use sea_orm::{DatabaseConnection, DbErr};
