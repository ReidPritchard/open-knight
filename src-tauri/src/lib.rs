use loader::load_pgn;
use models::api::AllValidMoves;
use shakmaty::san::San;
use state::AppState;

mod chess;
mod convert;
mod database;
mod db;
mod loader;
mod models;
mod parser;
mod state;

/// Error type for PGN parsing and processing
#[derive(Debug, serde::Serialize)]
pub enum PGNError {
    ParseError(String),
    DatabaseError(String),
    SerializationError(String),
}

impl std::fmt::Display for PGNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PGNError::ParseError(e) => write!(f, "Parse error: {}", e),
            PGNError::DatabaseError(e) => write!(f, "Database error: {}", e),
            PGNError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

#[tauri::command]
async fn empty_db(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state
        .clear()
        .await
        .map_err(|e| format!("Database error: {}", e))
}

#[tauri::command]
fn san_to_move(san: &str) -> String {
    let move_result: Result<San, _> = san.parse();
    match move_result {
        Ok(m) => format!("{:?}", m),
        Err(e) => format!("{:?}", e),
    }
}

#[tauri::command]
fn get_all_valid_moves(position: &str) -> Result<AllValidMoves, PGNError> {
    let valid_moves = chess::get_all_valid_moves(position);
    Ok(valid_moves)
}

#[tauri::command]
async fn parse_pgn(pgn: &str, state: tauri::State<'_, AppState>) -> Result<String, PGNError> {
    // Load and parse the PGN
    let load_result = load_pgn(pgn, state.get_db())
        .await
        .map_err(|e| PGNError::ParseError(e.to_string()))?;

    // Early return if there were parsing errors
    if !load_result.success {
        return Err(PGNError::ParseError(load_result.errors.join("\n")));
    }

    // load_pgn adds the games to the database
    // we should update the explorer state with the new games
    let mut explorer = state.explorer.lock().unwrap();
    explorer
        .load_games_from_db(state.get_db())
        .await
        .map_err(|e| PGNError::DatabaseError(e.to_string()))?;

    // Return a summary of what was parsed
    Ok(format!(
        "Successfully parsed {} games",
        load_result.games.len()
    ))
}

#[tauri::command]
fn get_explorer_state(state: tauri::State<'_, AppState>) -> String {
    let explorer = state.explorer.lock().unwrap().clone();
    serde_json::to_string_pretty(&explorer).unwrap()
}

#[tauri::command]
async fn set_selected_game(
    game_id: Option<i32>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    println!("Setting selected game to: {:?}", game_id);
    if let Some(game_id) = game_id {
        let api_game = database::game::get_full_game_by_id(state.get_db(), game_id)
            .await
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Game not found: {}", game_id))?;
        state.set_selected_game(Some(api_game));
    } else {
        state.set_selected_game(None);
    }
    Ok(())
}

#[tauri::command]
fn get_selected_game(state: tauri::State<'_, AppState>) -> String {
    let selected_game = state.selected_game.lock().unwrap().clone();
    serde_json::to_string_pretty(&selected_game).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .manage(
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(AppState::new())
                .expect("Failed to create AppState"),
        )
        .invoke_handler(tauri::generate_handler![
            san_to_move,
            get_all_valid_moves,
            parse_pgn,
            get_explorer_state,
            set_selected_game,
            get_selected_game,
            empty_db,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
