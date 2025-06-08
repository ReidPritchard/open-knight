use crate::api::commands::AppState;
use crate::utils::AppError;
use tauri::State;

/// Saves a game session to the database
///
/// Parameters:
/// - `board_id`: The ID of the board/session to save
/// - `overwrite`: Whether to overwrite existing game or create new one
///
/// Returns the ID of the saved game.
#[tauri::command]
pub async fn save_session(
    board_id: i32,
    overwrite: bool,
    state: State<'_, AppState>,
) -> Result<i32, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager
        .save_session(board_id, &state.db, overwrite)
        .await
    {
        Ok(game_id) => Ok(game_id),
        Err(e) => Err(AppError::DatabaseError(e.to_string())),
    }
}

/// Saves all active game sessions to the database
///
/// Parameters:
/// - `overwrite`: Whether to overwrite existing games or create new ones
///
/// Returns a JSON string containing the saved game IDs.
#[tauri::command]
pub async fn save_all_sessions(
    overwrite: bool,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager
        .save_all_sessions(&state.db, overwrite)
        .await
    {
        Ok(game_ids) => Ok(serde_json::to_string(&game_ids).unwrap()),
        Err(e) => Err(AppError::DatabaseError(e.to_string())),
    }
}
