use api::database::QueryParams;
use db::{connect_db, reset_database, run_migrations};
use sea_orm::DatabaseConnection;

pub mod api;
pub mod db;
pub mod entities;
pub mod migrations;
pub mod models;
pub mod parse;

/// Error type for PGN parsing and processing
#[derive(Debug, serde::Serialize)]
pub enum PGNError {
    ParseError(String),
    DatabaseError(String),
    SerializationError(String),
}

impl std::fmt::Display for PGNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PGNError::ParseError(e) => write!(f, "Parse error: {}", e),
            PGNError::DatabaseError(e) => write!(f, "Database error: {}", e),
            PGNError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

struct AppState {
    db: DatabaseConnection,
}

impl AppState {
    async fn new() -> Result<Self, PGNError> {
        let db = connect_db().await.unwrap();
        run_migrations(&db).await.unwrap();
        Ok(Self { db })
    }
}

#[tauri::command]
async fn empty_db(state: tauri::State<'_, AppState>) -> Result<(), String> {
    reset_database(&state.db).await.unwrap();
    Ok(())
}

#[tauri::command]
async fn parse_pgn(pgn: &str, state: tauri::State<'_, AppState>) -> Result<String, PGNError> {
    // Load and parse the PGN
    let load_result = models::ChessGame::save_from_pgn(&state.db, &pgn)
        .await
        .map_err(|e| PGNError::ParseError(e.to_string()))?;

    Ok(format!("Successfully parsed {} games", load_result.len()))
}

#[tauri::command]
async fn get_games(state: tauri::State<'_, AppState>) -> Result<String, PGNError> {
    let games = api::database::query_full_games(QueryParams::default(), &state.db)
        .await
        .unwrap();

    println!("Got {} games", games.len());

    Ok(serde_json::to_string(&games).unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(AppState::new())
                .expect("Failed to create AppState"),
        )
        .invoke_handler(tauri::generate_handler![parse_pgn, empty_db, get_games])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
