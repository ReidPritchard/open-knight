use convert::convert_to_games;
use loader::load_pgn;
use shakmaty::san::San;
use state::AppState;

mod convert;
mod database;
mod loader;
mod models;
mod schema;
mod state;

#[tauri::command]
fn empty_db(state: tauri::State<AppState>) {
    database::empty_db();

    // Reset the app state
    state.clear();
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

    // Convert to games and moves (for database insertion)
    let (games, moves) = convert_to_games(game_results.clone());

    // Insert into the database
    database::insert_games(&games);
    database::insert_moves(&moves);

    serde_json::to_string(&game_results).unwrap()
}

#[tauri::command]
fn get_explorer_state(state: tauri::State<AppState>) -> String {
    let explorer = state.explorer.lock().unwrap().clone();

    let explorer_json = serde_json::to_string_pretty(&explorer).unwrap();

    // println!("Explorer state: {}", explorer_json);

    explorer_json
}

#[tauri::command]
fn set_selected_game(game_id: Option<i32>, state: tauri::State<AppState>) {
    println!("Setting selected game: {}", game_id.unwrap_or(-1));
    if let Some(game_id) = game_id {
        let game_result = state.explorer.lock().unwrap().get_game_by_id(&game_id);
        state.set_selected_game(game_result);
    } else {
        state.set_selected_game(None);
    }
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
