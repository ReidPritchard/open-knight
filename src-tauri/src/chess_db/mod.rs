use sea_orm::*;
use sea_orm_migration::*;
use std::error::Error;

pub mod api;
pub mod entities;
pub mod migrations;
pub mod models;
pub mod parse;

// Constants
const DATABASE_URL: &str = "sqlite://chess.db?mode=rwc";
const DEBUG_PGN_FILE: &str = "./data/pgn/single-game.pgn";

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    migrations::Migrator::up(db, None).await?;
    Ok(())
}

pub async fn reset_database(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    migrations::Migrator::down(db, None).await?;
    migrations::Migrator::up(db, None).await?;
    Ok(())
}

pub async fn load_pgn_file(
    db: &DatabaseConnection,
    file_path: &str,
) -> Result<Vec<models::ChessGame>, Box<dyn Error>> {
    let pgn_content = std::fs::read_to_string(file_path)?;
    models::ChessGame::save_from_pgn(db, &pgn_content).await
}

pub async fn connect_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    Database::connect(DATABASE_URL).await.map_err(|e| e.into())
}

// Re-export commonly used types
pub use sea_orm::{DatabaseConnection, DbErr};
