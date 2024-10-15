use loader::load_pgn;
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

    format!("{:?}", game_results)
}

#[tauri::command]
fn get_games(pgn: &str) -> String {
    let load_result = load_pgn(pgn);
    format!("{:?}", load_result.games)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet, san_to_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
