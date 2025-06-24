use super::AppState;
use crate::engine::protocol::OptionValue;
use crate::utils::AppError;
use log::{debug, error};
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
    debug!("Loading engine: {}", name);

    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager
        .add_uci_engine(&name, &path, state.app_handle.clone())
        .await;
    drop(engine_manager);

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error loading engine: {}", e);
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
///
/// @deprecated: Use
#[tauri::command]
pub async fn analyze_position(
    fen: String,
    depth: Option<usize>,
    time_ms: Option<usize>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    debug!("Analyzing position");
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

/// Analyzes a chess position derived from a game session's current move/position
///
/// Parameters:
/// - `board_id`: The ID of the board to analyze
/// - `depth`: Optional depth limit for the analysis
/// - `time_ms`: Optional time limit in milliseconds
///
/// The analysis results will be sent through event listeners.
#[tauri::command]
pub async fn analyze_move(
    board_id: i32,
    depth: Option<usize>,
    time_ms: Option<usize>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // First get the game session
    let game_session_manager = state.game_session_manager.lock().await;
    let game_session = game_session_manager
        .get_session(board_id)
        .ok_or(AppError::SessionError("Game session not found".to_string()))
        .unwrap();

    // Then get the move from the game session
    let current_move_id = game_session.game.move_tree.current_node_id;
    let current_node = game_session.game.move_tree.nodes[current_move_id.unwrap()].clone();

    // Get the position from the current node
    let current_position = current_node.position;
    let fen = current_position.fen;

    // Set the position for the engine
    let mut engine_manager = state.engine_manager.lock().await;
    engine_manager
        .set_position(Some(&fen), None)
        .await
        .map_err(|e| e.to_string())?;

    // Start the analysis
    let result = engine_manager
        .start_analysis(depth.map(|d| d as u32), time_ms.map(|t| t as u32))
        .await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())

    // TODO: We need a way to store the evaluation results on the game move
    // I'm not sure if we should do this here/when the analysis events are emitted
    // or if we should let the client decide if they want to save the results
    // after they receive the events.
}

/// Analyzes a complete chess game from a game session
///
/// Parameters (TODO: Likely to change):
/// - `board_id`: The ID of the board to analyze
/// - `include_variations`: Whether to include variations in the analysis
///
#[tauri::command]
pub async fn analyze_game(
    _board_id: i32,
    _include_variations: bool,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    // Writing out my thoughts on how to implement this:
    // First get the game session's move tree

    // For each position/move, we need to run the engine to get the score and
    // best move. We need to be careful not to overload the engine with too many
    // requests.

    // We might want to add an argument for "average time" to analyze the game
    // for and use that to calculate how long to run the engine for each move
    // (time / number of moves).
    // Alternatively, we could just use a fixed time for each move.
    // We could also add an argument for a set depth for each move.

    // Once we have analyzed a move, we need to keep track of the results
    // and compare the score of the engine's best move with the score of the
    // move played. This difference will be used to categorize the move as
    // "brilliant", "best", "excellent", "good", "miss", "mistake", "blunder"
    // To do this we need a way to determine "meta" data about the game's state.
    // For example, if the move sacrifices material, we should consider that
    // when categorizing the move. This logic should be implemented separately
    // with the goal of being highly configurable and extensible.

    // Finally we need to save all of these results to the game's positions/moves.

    // We should split this into multiple functions to make things cleaner

    todo!("Implement analyze_game")
}

/// Stops any ongoing analysis
///
/// Terminates the current engine analysis and returns the engine to idle state.
#[tauri::command]
pub async fn stop_analysis(state: State<'_, AppState>) -> Result<(), String> {
    debug!("Stopping analysis");
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
