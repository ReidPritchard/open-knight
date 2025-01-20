use chess_db::{api, connect_db, reset_database};
use sea_orm::DatabaseConnection;

pub mod chess_db;

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
        Ok(Self { db })
    }
}

#[tauri::command]
async fn empty_db(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let db = connect_db().await.unwrap();
    reset_database(&db).await.unwrap();
    Ok(())
}

#[tauri::command]
async fn parse_pgn(pgn: &str, state: tauri::State<'_, AppState>) -> Result<String, PGNError> {
    // Load and parse the PGN
    let load_result = load_pgn(pgn, state.get_db())
        .await
        .map_err(|e| PGNError::ParseError(e.to_string()))?;

    // Early return if there were parsing errors
    if !load_result.success {
        return Err(PGNError::ParseError(load_result.errors.join("\n")));
    }

    // load_pgn adds the games to the database
    // we should update the explorer state with the new games
    let mut explorer = state.explorer.lock().unwrap();
    explorer
        .load_games_from_db(state.get_db())
        .await
        .map_err(|e| PGNError::DatabaseError(e.to_string()))?;

    // Return a summary of what was parsed
    Ok(format!(
        "Successfully parsed {} games",
        load_result.games.len()
    ))
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
        .invoke_handler(tauri::generate_handler![parse_pgn, empty_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
