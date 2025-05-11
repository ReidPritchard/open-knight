use api::database::QueryParams;
use db::{connect_db, reset_database, run_migrations};
use engine::utils::EngineError;
use engine::{manager::EngineManager, protocol::OptionValue};
use sea_orm::DatabaseConnection;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use utils::AppError;

pub mod api;
pub mod db;
pub mod engine;
pub mod entities;
pub mod macros;
pub mod migrations;
pub mod models;
pub mod parse;
pub mod utils;

struct AppState {
    app_handle: Arc<AppHandle>,
    db: DatabaseConnection,
    engine_manager: Mutex<EngineManager>,
}

impl AppState {
    async fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let db = connect_db().await.unwrap();
        run_migrations(&db).await.unwrap();
        Ok(Self {
            app_handle: Arc::new(app_handle),
            db,
            engine_manager: Mutex::new(EngineManager::new()),
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
) -> Result<String, AppError> {
    match models::ChessGame::save_from_pgn(&state.db, &pgn).await {
        Ok(games) => Ok(format!("Successfully parsed {} games", games.len())),
        Err(e) => Err(AppError::ParseError(e.to_string())),
    }
}

#[tauri::command]
async fn query_games(
    params: QueryParams,
    state: tauri::State<'_, AppState>,
) -> Result<String, AppError> {
    match api::database::query_full_games(params, &state.db).await {
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

#[tauri::command]
async fn query_entities(
    entity: &str,
    params: QueryParams,
    state: tauri::State<'_, AppState>,
) -> Result<String, AppError> {
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
) -> Result<String, AppError> {
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
) -> Result<String, AppError> {
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

#[tauri::command]
async fn analyze_position(
    fen: String,
    depth: Option<usize>,
    time_ms: Option<usize>,
    state: tauri::State<'_, AppState>,
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

#[tauri::command]
async fn analyze_game(_game_id: i32, _state: tauri::State<'_, AppState>) -> Result<(), String> {
    todo!("Implement analyze_game");
    // let mut params = QueryParams::default();
    // params.load_moves = Some(true);

    // let game = api::database::get_full_game(game_id, params, &state.db)
    //     .await
    //     .unwrap();
    // if let Some(mut game) = game {
    //     let engine_manager = state.engine_manager.lock().unwrap();
    //     let analysis_result = engine_manager.analyze_game(&engine_name, &mut game);
    //     drop(engine_manager); // Drop mutex guard before async operation
    //     match analysis_result {
    //         Ok(_) => Ok(()),
    //         Err(e) => Err(e.to_string()),
    //     }
    // } else {
    //     Err("Game not found".to_string())
    // }
}

#[tauri::command]
async fn stop_analysis(state: tauri::State<'_, AppState>) -> Result<(), String> {
    println!("Stopping analysis");
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.stop_analysis().await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_engine_option(
    engine_name: String,
    option: String,
    value: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager
        .set_engine_option(&engine_name, &option, OptionValue::String(value))
        .await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_position(fen: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.set_position(Some(&fen), None).await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_legal_moves(fen: String) -> Result<String, AppError> {
    match api::chess::get_legal_moves(&fen) {
        Ok(moves) => match serde_json::to_string(&moves) {
            Ok(json) => Ok(json),
            Err(e) => Err(AppError::SerializationError(e.to_string())),
        },
        Err(e) => Err(AppError::ChessError(e.to_string())),
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
            analyze_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
