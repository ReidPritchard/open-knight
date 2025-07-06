use crate::api::commands::AppState;
use crate::api::database::QueryParams;
use crate::models;
use crate::utils::AppError;
use tauri::State;

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
    params.load_headers = Some(true);

    let game =
        crate::api::database::get_full_game(game_id, params, &state.db).await;

    match game {
        Ok(Some(game)) => {
            let mut game_session_manager =
                state.game_session_manager.lock().await;
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
pub async fn get_session(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
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
pub async fn get_all_sessions(
    state: State<'_, AppState>
) -> Result<String, AppError> {
    let game_session_manager = state.game_session_manager.lock().await;
    let sessions = game_session_manager.get_all_sessions();
    Ok(serde_json::to_string(&sessions).unwrap())
}

/// Closes a game session
///
/// Parameters:
/// - `board_id`: The ID of the board to close the session on
#[tauri::command]
pub async fn close_session(
    board_id: i32,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    game_session_manager.close_session(board_id);
    Ok(())
}

/// Closes all active game sessions
#[tauri::command]
pub async fn close_all_sessions(
    state: State<'_, AppState>
) -> Result<(), AppError> {
    let mut game_session_manager = state.game_session_manager.lock().await;
    game_session_manager.close_all_sessions();
    Ok(())
}
