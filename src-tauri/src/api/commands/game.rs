use super::AppState;
use crate::api::database::QueryParams;
use crate::models;
use crate::utils::AppError;
use tauri::State;

// =============================================================================
// SESSION LIFECYCLE MANAGEMENT
// =============================================================================

/// Creates a new game session with a fresh game
///
/// Parameters:
/// - `board_id`: The ID of the board to create the session on
/// - `variant`: The chess variant to create (e.g., "standard")
///
/// Returns a JSON string containing the new game session.
#[tauri::command]
pub async fn create_session(
    board_id: i32,
    variant: &str,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let game = match models::ChessGame::new(variant, &state.db).await {
        Ok(game) => game,
        Err(e) => return Err(e),
    };

    let mut game_session_manager = state.game_session_manager.lock().await;
    game_session_manager.add_game(game.clone(), board_id);
    Ok(serde_json::to_string(&game).unwrap())
}

/// Loads an existing game from the database into a session
///
/// Parameters:
/// - `game_id`: The ID of the game to load
/// - `board_id`: The ID of the board to load the game on
#[tauri::command]
pub async fn load_game_into_session(
    game_id: i32,
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let mut params = QueryParams::default();
    params.load_moves = Some(true);
    params.load_tags = Some(true);

    let game = crate::api::database::get_full_game(game_id, params, &state.db).await;

    match game {
        Ok(Some(game)) => {
            let mut game_session_manager = state.game_session_manager.lock().await;
            game_session_manager.add_game(game.clone(), board_id);
            Ok(serde_json::to_string(&game).unwrap())
        }
        _ => Err(AppError::DatabaseError(
            "Failed to load game into session".to_string(),
        )),
    }
}

/// Gets the current state of a specific game session
///
/// Parameters:
/// - `board_id`: The ID of the board/session to retrieve
///
/// Returns a JSON string containing the current game state.
#[tauri::command]
pub async fn get_session(board_id: i32, state: State<'_, AppState>) -> Result<String, AppError> {
    let game_session_manager = state.game_session_manager.lock().await;
    match game_session_manager.get_session(board_id) {
        Some(session) => Ok(serde_json::to_string(&session.game).unwrap()),
        None => Err(AppError::SessionError(format!(
            "No session found for board {}",
            board_id
        ))),
    }
}

/// Gets all active game sessions
///
/// Returns a JSON string containing all active sessions with their board IDs.
#[tauri::command]
pub async fn get_all_sessions(state: State<'_, AppState>) -> Result<String, AppError> {
    let game_session_manager = state.game_session_manager.lock().await;
    let sessions = game_session_manager.get_all_sessions();
    Ok(serde_json::to_string(&sessions).unwrap())
}

/// Closes a game session
///
/// Parameters:
/// - `board_id`: The ID of the board to close the session on
#[tauri::command]
pub async fn close_session(board_id: i32, state: State<'_, AppState>) -> Result<(), AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    game_session_manager.close_session(board_id);
    Ok(())
}

/// Closes all active game sessions
#[tauri::command]
pub async fn close_all_sessions(state: State<'_, AppState>) -> Result<(), AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    game_session_manager.close_all_sessions();
    Ok(())
}

// =============================================================================
// SESSION OPERATIONS
// =============================================================================

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
pub async fn undo_move(board_id: i32, state: State<'_, AppState>) -> Result<String, AppError> {
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
pub async fn redo_move(board_id: i32, state: State<'_, AppState>) -> Result<String, AppError> {
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
pub async fn previous_move(board_id: i32, state: State<'_, AppState>) -> Result<String, AppError> {
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
/// - `move_number`: The move number to reset to (0 = start position)
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

// =============================================================================
// SESSION PERSISTENCE
// =============================================================================

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
