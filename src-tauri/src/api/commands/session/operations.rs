use crate::utils::AppError;
use crate::AppState;
use tauri::State;

/// Makes a move in a game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session
/// - `move_notation`: The move in algebraic notation
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn make_move(
    board_id: i32,
    move_notation: &str,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager
        .make_move(board_id, move_notation)
        .await
    {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(e),
    }
}

/// Undoes the last move in a game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn undo_move(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.undo_move(board_id) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Redoes a previously undone move in a game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn redo_move(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.redo_move(board_id) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Move to the next move in a game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session
/// - `variation`: The variation index to move to (optional, 0 = main line)
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn next_move(
    board_id: i32,
    variation: usize,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.next_move(board_id, variation) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Move to the previous move in a game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn previous_move(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.previous_move(board_id) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Resets a game session to a specific position/move number
///
/// Parameters:
/// - `board_id`: The ID of the board/session
/// - `move_db_id`: The database ID of the move to reset to
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn reset_to_position(
    board_id: i32,
    move_db_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.reset_to_position(board_id, move_db_id) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Navigate to the start (root) of the game
///
/// Parameters:
/// - `board_id`: The ID of the board/session
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn navigate_to_start(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.navigate_to_start(board_id) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Navigate to the end of the current main line
///
/// Parameters:
/// - `board_id`: The ID of the board/session
///
/// Returns a JSON string containing the updated game state.
#[tauri::command]
pub async fn navigate_to_end(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.navigate_to_end(board_id) {
        Ok(_) => {
            let session = game_session_manager.get_session(board_id).unwrap();
            Ok(serde_json::to_string(&session.game).unwrap())
        }
        Err(e) => Err(AppError::SessionError(e.to_string())),
    }
}

/// Gets the move history for a game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session
///
/// Returns a JSON string containing the move history.
#[tauri::command]
pub async fn get_session_moves(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.get_session(board_id) {
        Some(session) => {
            let moves = session.get_move_history();
            Ok(serde_json::to_string(&moves).unwrap())
        }
        None => Err(AppError::SessionError(format!(
            "No session found for board {}",
            board_id
        ))),
    }
}
