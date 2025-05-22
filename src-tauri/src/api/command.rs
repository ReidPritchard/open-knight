use crate::api::database::QueryParams;
use crate::db::{connect_db, reset_database, run_migrations};
use crate::engine::{manager::EngineManager, protocol::OptionValue};
use crate::models;
use crate::utils::AppError;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::Mutex;

/// Application state shared across Tauri commands
///
/// Contains:
/// - The Tauri application handle
/// - Database connection
/// - Engine manager to interact with chess engines
pub struct AppState {
    pub app_handle: Arc<AppHandle>,
    pub db: DatabaseConnection,
    pub engine_manager: Mutex<EngineManager>,
}

impl AppState {
    /// Creates a new AppState instance with initialized database connection and engine manager
    pub async fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let db = connect_db().await.unwrap();
        run_migrations(&db).await.unwrap();
        Ok(Self {
            app_handle: Arc::new(app_handle),
            db,
            engine_manager: Mutex::new(EngineManager::new()),
        })
    }
}

// Database commands

/// Resets the database to its initial empty state
///
/// This command removes all data from the database.
/// Useful for testing and development.
#[tauri::command]
pub async fn empty_db(state: State<'_, AppState>) -> Result<(), String> {
    reset_database(&state.db).await.unwrap();
    Ok(())
}

/// Imports chess games from PGN format
///
/// Parses the provided PGN string and saves the games to the database.
/// Returns the number of successfully parsed games.
#[tauri::command]
pub async fn import_pgn_games(pgn: &str, state: State<'_, AppState>) -> Result<String, AppError> {
    match models::ChessGame::save_from_pgn(&state.db, &pgn).await {
        Ok(games) => Ok(format!("Successfully parsed {} games", games.len())),
        Err(e) => Err(AppError::ParseError(e.to_string())),
    }
}

/// Queries chess games from the database based on provided parameters
///
/// Returns a JSON string containing the matching games.
#[tauri::command]
pub async fn query_games(
    params: QueryParams,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    match crate::api::database::query_full_games(params, &state.db).await {
        Ok(games) => {
            println!("Successfully retrieved {} games from database", games.len());
            match serde_json::to_string(&games) {
                Ok(json) => Ok(json),
                Err(e) => {
                    eprintln!("Error serializing games to JSON: {}", e);
                    Err(AppError::SerializationError(e.to_string()))
                }
            }
        }
        Err(e) => {
            eprintln!("Error querying games from database: {}", e);
            Err(AppError::DatabaseError(format!(
                "Failed to query games: {}",
                e
            )))
        }
    }
}

/// Queries entities of a specific type from the database
///
/// Parameters:
/// - `entity`: The type of entity to query (e.g., "player", "event")
/// - `params`: Query parameters for filtering
///
/// Returns a JSON string containing the matching entities.
#[tauri::command]
pub async fn query_entities(
    entity: &str,
    params: QueryParams,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let games = crate::api::database::query_entities(entity, params, &state.db)
        .await
        .unwrap();

    Ok(serde_json::to_string(&games).unwrap())
}

/// Retrieves a specific entity by its ID
///
/// Parameters:
/// - `entity`: The type of entity to retrieve (e.g., "player", "event")
/// - `id`: The ID of the entity
///
/// Returns a JSON string containing the entity data.
#[tauri::command]
pub async fn get_entity_by_id(
    entity: &str,
    id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let entity = crate::api::database::get_entity_by_id(entity, id, None, &state.db)
        .await
        .unwrap();
    Ok(serde_json::to_string(&entity).unwrap())
}

/// Retrieves a specific chess game by its ID
///
/// Parameters:
/// - `id`: The ID of the game to retrieve
/// - `params`: Query parameters for controlling what data to load
///
/// Returns a JSON string containing the full game data.
#[tauri::command]
pub async fn get_game_by_id(
    id: i32,
    params: QueryParams,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let game = crate::api::database::get_full_game(id, params, &state.db)
        .await
        .unwrap();
    Ok(serde_json::to_string(&game).unwrap())
}

/// Gets all legal moves for a given chess position
///
/// Parameters:
/// - `fen`: The FEN string representing the position
///
/// Returns a JSON string containing all legal moves.
#[tauri::command]
pub async fn get_legal_moves(fen: String) -> Result<String, AppError> {
    match crate::api::chess::get_legal_moves(&fen) {
        Ok(moves) => match serde_json::to_string(&moves) {
            Ok(json) => Ok(json),
            Err(e) => Err(AppError::SerializationError(e.to_string())),
        },
        Err(e) => Err(AppError::ChessError(e.to_string())),
    }
}

// Engine commands

/// Gets the state of all loaded chess engines
///
/// Returns a JSON string containing the state of all engines.
#[tauri::command]
pub async fn get_all_engine_state(state: State<'_, AppState>) -> Result<String, String> {
    let engine_manager = state.engine_manager.lock().await;
    let states = engine_manager.get_all_engine_state().await;
    Ok(serde_json::to_string(&states).unwrap())
}

/// Loads a UCI chess engine
///
/// Parameters:
/// - `name`: A unique name for the engine
/// - `path`: The file path to the engine executable
///
/// The engine will be initialized and made available for analysis.
#[tauri::command]
pub async fn load_engine(
    name: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    println!("Loading engine: {}", name);

    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager
        .add_uci_engine(&name, &path, state.app_handle.clone())
        .await;
    drop(engine_manager);

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error loading engine: {}", e);
            Err(e.to_string())
        }
    }
}

/// Unloads a previously loaded chess engine
///
/// Parameters:
/// - `name`: The name of the engine to unload
#[tauri::command]
pub async fn unload_engine(name: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.remove_engine(&name).await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Analyzes a chess position with the loaded engine
///
/// Parameters:
/// - `fen`: The FEN string representing the position to analyze
/// - `depth`: Optional depth limit for the analysis
/// - `time_ms`: Optional time limit in milliseconds
///
/// The analysis results will be sent through event listeners.
#[tauri::command]
pub async fn analyze_position(
    fen: String,
    depth: Option<usize>,
    time_ms: Option<usize>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    println!("Analyzing position");
    let mut engine_manager = state.engine_manager.lock().await;

    engine_manager
        .set_position(Some(&fen), None)
        .await
        .map_err(|e| e.to_string())?;

    let result = engine_manager
        .start_analysis(depth.map(|d| d as u32), time_ms.map(|t| t as u32))
        .await;

    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Analyzes a complete chess game
///
/// Parameters:
/// - `game_id`: The ID of the game to analyze
///
/// Not yet implemented.
#[tauri::command]
pub async fn analyze_game(_game_id: i32, _state: State<'_, AppState>) -> Result<(), String> {
    todo!("Implement analyze_game")
}

/// Stops any ongoing analysis
///
/// Terminates the current engine analysis and returns the engine to idle state.
#[tauri::command]
pub async fn stop_analysis(state: State<'_, AppState>) -> Result<(), String> {
    println!("Stopping analysis");
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.stop_analysis().await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Sets an option for a specific chess engine
///
/// Parameters:
/// - `engine_name`: The name of the engine
/// - `option`: The name of the option to set
/// - `value`: The value to set for the option
#[tauri::command]
pub async fn set_engine_option(
    engine_name: String,
    option: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager
        .set_engine_option(&engine_name, &option, OptionValue::String(value))
        .await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Sets the current position for the engine
///
/// Parameters:
/// - `fen`: The FEN string representing the position to set
#[tauri::command]
pub async fn set_position(fen: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.set_position(Some(&fen), None).await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}
