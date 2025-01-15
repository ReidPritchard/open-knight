use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use std::error::Error;

pub mod api;
pub mod migrations;
pub mod models;
pub mod parse;

pub const DEBUG: bool = true;
pub const DATABASE_URL: &str = "sqlite://database.sqlite?mode=rwc";
pub const _DATABASE_DEBUG_URL: &str = "sqlite::memory:";
pub const DEBUG_PGN_FILE: &str = "./src/data/pgn/single-game.pgn";

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    println!("Running migrations...");
    migrations::Migrator::up(db, None).await?;
    Ok(())
}

pub async fn reset_database(db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
    println!("Resetting database...");
    migrations::Migrator::down(db, None).await?;
    migrations::Migrator::up(db, None).await?;
    Ok(())
}

pub async fn load_pgn_file(
    db: &DatabaseConnection,
    file_path: &str,
) -> Result<Vec<models::ChessGame>, Box<dyn Error>> {
    println!("Loading PGN file from {}", file_path);
    let pgn_content = std::fs::read_to_string(file_path)?;
    println!("PGN file read successfully");
    println!("Saving games to database...");
    let games = models::ChessGame::save_from_pgn(db, &pgn_content).await?;
    println!("Games saved successfully");

    Ok(games)
}

pub async fn connect_db() -> Result<DatabaseConnection, Box<dyn Error>> {
    println!("Connecting to database...");
    let db: DatabaseConnection = Database::connect(DATABASE_URL).await?;
    Ok(db)
}
