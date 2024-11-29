use crate::models::game::ExplorerGame;
use convert::parsing_games_to_models;
use loader::load_pgn;
use shakmaty::san::San;
use state::AppState;

mod convert;
mod database;
mod loader;
mod models;
mod parser;
mod schema;
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
fn empty_db(state: tauri::State<AppState>) -> Result<(), String> {
    database::setup::empty_db().map_err(|e| format!("Database error: {}", e))?;

    // Reset the app state
    state.clear();
    Ok(())
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
fn parse_pgn(pgn: &str, state: tauri::State<AppState>) -> Result<String, String> {
    // Load and parse the PGN
    let load_result = load_pgn(pgn).map_err(|e| e.to_string())?;

    // Early return if there were parsing errors
    if !load_result.success {
        return Err(load_result.errors.join("\n"));
    }

    let parsing_games = load_result.get_games();

    // Process the games in a transaction to ensure database consistency
    let result = || -> Result<String, String> {
        // Convert to database models
        let (games, moves, headers) = parsing_games_to_models(parsing_games.clone());

        // Insert into database
        database::game::insert_games(&games).map_err(|e| format!("Database error: {}", e))?;
        database::move_::insert_moves(&moves).map_err(|e| format!("Database error: {}", e))?;
        database::header::insert_headers(&headers).map_err(|e| format!("Database error: {}", e))?;

        // Get all games with their headers in a single query
        let games_with_headers = database::game::get_all_games_with_headers()
            .map_err(|e| format!("Database error: {}", e))?;

        // Convert to explorer games
        let explorer_games = games_with_headers
            .into_iter()
            .map(|(game, headers)| ExplorerGame::from((game, headers)))
            .collect::<Vec<_>>();

        // Update application state
        state
            .explorer
            .lock()
            .map_err(|e| format!("State error: {}", e))?
            .extend(&explorer_games);

        // Serialize the result
        serde_json::to_string(&explorer_games).map_err(|e| format!("Serialization error: {}", e))
    }();

    result
}

#[tauri::command]
fn get_explorer_state(state: tauri::State<AppState>) -> String {
    let explorer = state.explorer.lock().unwrap().clone();
    serde_json::to_string_pretty(&explorer).unwrap()
}

#[tauri::command]
fn set_selected_game(game_id: Option<i32>, state: tauri::State<AppState>) -> Result<(), String> {
    println!("Setting selected game to: {:?}", game_id);
    if let Some(game_id) = game_id {
        let api_game = database::game::get_full_game_by_id(game_id)
            .map_err(|e| format!("Database error: {}", e))?
            .ok_or_else(|| format!("Game not found: {}", game_id))?;
        state.set_selected_game(Some(api_game));
    } else {
        state.set_selected_game(None);
    }
    Ok(())
}

#[tauri::command]
fn get_selected_game(state: tauri::State<AppState>) -> String {
    let selected_game = state.selected_game.lock().unwrap().clone();
    serde_json::to_string_pretty(&selected_game).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            san_to_move,
            parse_pgn,
            get_explorer_state,
            set_selected_game,
            get_selected_game,
            empty_db,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
