use api::command::AppState;
use tauri::Manager;

pub mod api;
pub mod db;
pub mod engine;
pub mod entities;
pub mod macros;
pub mod migrations;
pub mod models;
pub mod parse;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            app.manage(
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(AppState::new(app_handle.clone()))
                    .expect("Failed to create AppState"),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Database commands
            api::command::import_pgn_games,
            api::command::empty_db,
            api::command::query_games,
            api::command::query_entities,
            api::command::get_entity_by_id,
            api::command::get_game_by_id,
            api::command::get_legal_moves,
            // Engine commands
            api::command::load_engine,
            api::command::unload_engine,
            api::command::analyze_position,
            api::command::stop_analysis,
            api::command::set_engine_option,
            api::command::set_position,
            api::command::analyze_game,
            api::command::get_all_engine_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
