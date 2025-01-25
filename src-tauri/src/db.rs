use sea_orm::*;
use sea_orm_migration::*;
use std::error::Error;

// Constants
const DATABASE_URL: &str = "sqlite://chess.db?mode=rwc";
const DEBUG_PGN_FILE: &str = "./src/data/pgn/my-games.pgn";

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    Migrator::up(db, None).await?;
    Ok(())
}

pub async fn reset_database(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    Migrator::down(db, None).await?;
    Migrator::up(db, None).await?;
    Ok(())
}

pub async fn load_pgn_file(
    db: &DatabaseConnection,
    file_path: &str,
) -> Result<Vec<models::ChessGame>, Box<dyn Error>> {
    let pgn_content = std::fs::read_to_string(file_path)?;
    models::ChessGame::save_from_pgn(db, &pgn_content).await
}

pub async fn import_demo_games(
    db: &DatabaseConnection,
) -> Result<Vec<models::ChessGame>, Box<dyn Error>> {
    let pgn_content = std::fs::read_to_string(DEBUG_PGN_FILE)?;
    models::ChessGame::save_from_pgn(db, &pgn_content).await
}

pub async fn connect_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    Database::connect(DATABASE_URL).await.map_err(|e| e.into())
}

// Re-export commonly used types
pub use sea_orm::{DatabaseConnection, DbErr};

use crate::{migrations::Migrator, models};
