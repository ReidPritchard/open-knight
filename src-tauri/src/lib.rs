use api::database::QueryParams;
use db::{connect_db, reset_database, run_migrations};
use engine::engine::EngineError;
use engine::manager::EngineManager;
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub mod api;
pub mod db;
pub mod engine;
pub mod entities;
pub mod macros;
pub mod migrations;
pub mod models;
pub mod parse;

/// Error type for PGN parsing and processing
/// FIXME: Move this to the PGN parsing crate
/// and remove the unrelated error types
#[derive(Debug, serde::Serialize)]
pub enum PGNError {
    ParseError(String),
    DatabaseError(String),
    SerializationError(String),
    ChessError(String),
    EngineError(String),
}

impl std::fmt::Display for PGNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PGNError::ParseError(e) => write!(f, "Parse error: {}", e),
            PGNError::DatabaseError(e) => write!(f, "Database error: {}", e),
            PGNError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            PGNError::ChessError(e) => write!(f, "Chess error: {}", e),
            PGNError::EngineError(e) => write!(f, "Engine error: {}", e),
        }
    }
}

struct AppState {
    db: DatabaseConnection,
    engine_manager: Mutex<EngineManager>,
}

impl AppState {
    async fn new(app_handle: AppHandle) -> Result<Self, PGNError> {
        let db = connect_db().await.unwrap();
        run_migrations(&db).await.unwrap();
        Ok(Self {
            db,
            engine_manager: Mutex::new(EngineManager::new(app_handle, None)),
        })
    }
}

#[tauri::command]
async fn empty_db(state: tauri::State<'_, AppState>) -> Result<(), String> {
    reset_database(&state.db).await.unwrap();
    Ok(())
}

#[tauri::command]
async fn import_pgn_games(
    pgn: &str,
    state: tauri::State<'_, AppState>,
) -> Result<String, PGNError> {
    match models::ChessGame::save_from_pgn(&state.db, &pgn).await {
        Ok(games) => Ok(format!("Successfully parsed {} games", games.len())),
        Err(e) => Err(PGNError::ParseError(e.to_string())),
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
async fn load_engine(
    name: String,
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, EngineError> {
    let mut engine_manager = state.engine_manager.lock().unwrap();
    match engine_manager.load_engine(name, PathBuf::from(path)) {
        Ok(_) => Ok("Engine loaded".to_string()),
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn analyze_position(
    engine_name: String,
    fen: String,
    depth: Option<usize>,
    time_ms: Option<usize>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    println!("Analyzing position");
    let engine_manager = state.engine_manager.lock().unwrap();
    if let Some(engine) = engine_manager.get_engine(&engine_name) {
        let mut engine = engine
            .lock()
            .map_err(|_| "Failed to lock engine".to_string())?;

        // Set the position
        engine.position_from_fen(&fen)?;

        // Start analysis
        if let Some(depth) = depth {
            println!("Trying to go depth: {}", depth);
            engine.go_depth(depth)?;
        } else if let Some(time_ms) = time_ms {
            println!("Trying to go movetime: {}", time_ms);
            engine.go_movetime(time_ms)?;
        } else {
            println!("Trying to go infinite");
            engine.go_infinite()?;
        }

        Ok(())
    } else {
        Err(format!("Engine '{}' not found", engine_name))
    }
}

#[tauri::command]
async fn analyze_game(
    engine_name: String,
    game_id: i32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut params = QueryParams::default();
    params.load_moves = Some(true);

    let game = api::database::get_full_game(game_id, params, &state.db)
        .await
        .unwrap();
    if let Some(mut game) = game {
        let engine_manager = state.engine_manager.lock().unwrap();
        let analysis_result = engine_manager.analyze_game(&engine_name, &mut game);
        drop(engine_manager); // Drop mutex guard before async operation
        match analysis_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("Game not found".to_string())
    }
}

#[tauri::command]
async fn stop_analysis(
    engine_name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    println!("Stopping analysis");
    let engine_manager = state.engine_manager.lock().unwrap();
    if let Some(engine) = engine_manager.get_engine(&engine_name) {
        let mut engine = engine
            .lock()
            .map_err(|_| "Failed to lock engine".to_string())?;
        engine.stop().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Engine '{}' not found", engine_name))
    }
}

#[tauri::command]
async fn set_engine_option(
    engine_name: String,
    option: String,
    value: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let engine_manager = state.engine_manager.lock().unwrap();
    if let Some(engine) = engine_manager.get_engine(&engine_name) {
        let mut engine = engine
            .lock()
            .map_err(|_| "Failed to lock engine".to_string())?;
        engine.set_option(&option, &value)?;
        Ok(())
    } else {
        Err(format!("Engine '{}' not found", engine_name))
    }
}

#[tauri::command]
async fn set_position(
    engine_name: String,
    fen: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let engine_manager = state.engine_manager.lock().unwrap();
    if let Some(engine) = engine_manager.get_engine(&engine_name) {
        let mut engine = engine
            .lock()
            .map_err(|_| "Failed to lock engine".to_string())?;
        engine.position_from_fen(&fen)?;
        Ok(())
    } else {
        Err(format!("Engine '{}' not found", engine_name))
    }
}

#[tauri::command]
async fn go_depth(
    engine_name: String,
    depth: usize,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let engine_manager = state.engine_manager.lock().unwrap();
    if let Some(engine) = engine_manager.get_engine(&engine_name) {
        let mut engine = engine
            .lock()
            .map_err(|_| "Failed to lock engine".to_string())?;
        engine.go_depth(depth)?;
        Ok(())
    } else {
        Err(format!("Engine '{}' not found", engine_name))
    }
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
        .setup(|app| {
            let app_handle = app.handle();
            app.manage(
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(AppState::new(app_handle.clone()))
                    .expect("Failed to create AppState"),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Database commands
            import_pgn_games,
            empty_db,
            query_games,
            query_entities,
            get_entity_by_id,
            get_game_by_id,
            get_legal_moves,
            // Engine commands
            load_engine,
            analyze_position,
            stop_analysis,
            set_engine_option,
            set_position,
            go_depth,
            analyze_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
