use api::database::QueryParams;
use db::{connect_db, reset_database, run_migrations};
use sea_orm::DatabaseConnection;

pub mod api;
pub mod db;
pub mod entities;
pub mod macros;
pub mod migrations;
pub mod models;
pub mod parse;

/// Error type for PGN parsing and processing
#[derive(Debug, serde::Serialize)]
pub enum PGNError {
    ParseError(String),
    DatabaseError(String),
    SerializationError(String),
    ChessError(String),
}

impl std::fmt::Display for PGNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PGNError::ParseError(e) => write!(f, "Parse error: {}", e),
            PGNError::DatabaseError(e) => write!(f, "Database error: {}", e),
            PGNError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            PGNError::ChessError(e) => write!(f, "Chess error: {}", e),
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
async fn import_demo_games(state: tauri::State<'_, AppState>) -> Result<String, PGNError> {
    match db::import_demo_games(&state.db).await {
        Ok(games) => {
            println!("Successfully imported {} demo games", games.len());
            Ok(format!("Successfully imported {} games", games.len()))
        }
        Err(e) => {
            eprintln!("Error importing demo games: {}", e);
            Err(PGNError::DatabaseError(format!(
                "Failed to import demo games: {}",
                e
            )))
        }
    }
}

#[tauri::command]
async fn query_games(
    params: QueryParams,
    state: tauri::State<'_, AppState>,
) -> Result<String, PGNError> {
    match api::database::query_full_games(params, &state.db).await {
        Ok(games) => {
            println!("Successfully retrieved {} games from database", games.len());
            match serde_json::to_string(&games) {
                Ok(json) => Ok(json),
                Err(e) => {
                    eprintln!("Error serializing games to JSON: {}", e);
                    Err(PGNError::SerializationError(e.to_string()))
                }
            }
        }
        Err(e) => {
            eprintln!("Error querying games from database: {}", e);
            Err(PGNError::DatabaseError(format!(
                "Failed to query games: {}",
                e
            )))
        }
    }
}

#[tauri::command]
async fn query_entities(
    entity: &str,
    params: QueryParams,
    state: tauri::State<'_, AppState>,
) -> Result<String, PGNError> {
    let games = api::database::query_entities(entity, params, &state.db)
        .await
        .unwrap();

    Ok(serde_json::to_string(&games).unwrap())
}

#[tauri::command]
async fn get_entity_by_id(
    entity: &str,
    id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<String, PGNError> {
    let entity = api::database::get_entity_by_id(entity, id, None, &state.db)
        .await
        .unwrap();
    Ok(serde_json::to_string(&entity).unwrap())
}

#[tauri::command]
async fn get_game_by_id(
    id: i32,
    params: QueryParams,
    state: tauri::State<'_, AppState>,
) -> Result<String, PGNError> {
    let game = api::database::get_full_game(id, params, &state.db)
        .await
        .unwrap();
    Ok(serde_json::to_string(&game).unwrap())
}

#[tauri::command]
async fn get_legal_moves(fen: String) -> Result<String, PGNError> {
    match api::chess::get_legal_moves(&fen) {
        Ok(moves) => match serde_json::to_string(&moves) {
            Ok(json) => Ok(json),
            Err(e) => Err(PGNError::SerializationError(e.to_string())),
        },
        Err(e) => Err(PGNError::ChessError(e.to_string())),
    }
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
        .invoke_handler(tauri::generate_handler![
            parse_pgn,
            empty_db,
            import_demo_games,
            query_games,
            query_entities,
            get_entity_by_id,
            get_game_by_id,
            get_legal_moves,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
