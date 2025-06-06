use api::AppState;
use tauri::Manager;

pub mod api;
pub mod db;
pub mod engine;
pub mod entities;
pub mod macros;
pub mod migrations;
pub mod models;
pub mod parse;
pub mod session;
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
            api::commands::database::import_pgn_games,
            api::commands::database::empty_db,
            api::commands::database::query_games,
            api::commands::database::query_entities,
            api::commands::database::get_entity_by_id,
            api::commands::database::get_game_by_id,
            api::commands::database::delete_game,
            api::commands::database::update_game_property,
            // Session lifecycle commands
            api::commands::game::create_session,
            api::commands::game::load_game_into_session,
            api::commands::game::get_session,
            api::commands::game::get_all_sessions,
            api::commands::game::close_session,
            api::commands::game::close_all_sessions,
            // Session operation commands
            api::commands::game::make_move,
            api::commands::game::undo_move,
            api::commands::game::redo_move,
            api::commands::game::next_move,
            api::commands::game::previous_move,
            api::commands::game::navigate_to_end,
            api::commands::game::navigate_to_start,
            api::commands::game::reset_to_position,
            api::commands::game::get_session_moves,
            // Session persistence commands
            api::commands::game::save_session,
            api::commands::game::save_all_sessions,
            // Chess commands
            api::commands::chess::get_legal_moves,
            // Engine commands
            api::commands::engine::load_engine,
            api::commands::engine::unload_engine,
            api::commands::engine::analyze_position,
            api::commands::engine::stop_analysis,
            api::commands::engine::set_engine_option,
            api::commands::engine::set_position,
            api::commands::engine::analyze_game,
            api::commands::engine::get_all_engine_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
