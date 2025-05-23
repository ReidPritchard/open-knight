use super::AppState;
use crate::engine::protocol::OptionValue;
use tauri::State;

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
