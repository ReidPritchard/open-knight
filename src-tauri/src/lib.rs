use loader::{load_pgn, GameResult};
use shakmaty::san::San;
use state::AppState;

mod loader;
mod state;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn san_to_move(san: &str) -> String {
    let move_result: Result<San, _> = san.parse();

    let move_result = match move_result {
        Ok(m) => format!("{:?}", m),
        Err(e) => format!("{:?}", e),
    };

    move_result
}

#[tauri::command]
fn parse_pgn(pgn: &str, state: tauri::State<AppState>) -> String {
    let load_result = load_pgn(pgn);
    let game_results = load_result.get_game_results();

    // Save the result to the app state
    state.explorer.lock().unwrap().extend(&game_results);

    serde_json::to_string(&game_results).unwrap()
}

#[tauri::command]
fn get_explorer_state(state: tauri::State<AppState>) -> String {
    let explorer = state.explorer.lock().unwrap().clone();

    let explorer_json = serde_json::to_string_pretty(&explorer).unwrap();

    explorer_json
}

#[tauri::command]
fn set_selected_game(game_id: String, state: tauri::State<AppState>) {
    let game_result = state.explorer.lock().unwrap().get_game_by_id(&game_id);
    state.set_selected_game(game_result);
}

#[tauri::command]
fn get_selected_game(state: tauri::State<AppState>) -> String {
    let selected_game = state.selected_game.lock().unwrap().clone();
    let selected_game_json = serde_json::to_string_pretty(&selected_game).unwrap();

    selected_game_json
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            san_to_move,
            parse_pgn,
            get_explorer_state,
            set_selected_game,
            get_selected_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
