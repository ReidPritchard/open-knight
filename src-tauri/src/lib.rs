use api::AppState;
use log::LevelFilter;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub mod api;
pub mod db;
pub mod engine;
pub mod entities;
pub mod macros;
pub mod migrations;
pub mod models;
pub mod session;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::Stdout))
                .level(LevelFilter::max())
                .level_for("sea_orm", LevelFilter::Off)
                .level_for("sqlx", LevelFilter::Off)
                .level_for("tracing", LevelFilter::Off)
                .level_for("tao", LevelFilter::Off)
                .build(),
        )
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
            // Database management commands
            api::commands::database::import_pgn_games,
            api::commands::database::empty_db,
            api::commands::database::import_eco_database,
            // Database game commands
            api::commands::database::query_games,
            api::commands::database::get_game_by_id,
            api::commands::database::delete_game,
            api::commands::database::update_game_property,
            // Database entity commands
            api::commands::database::query_entities,
            api::commands::database::get_entity_by_id,
            // Session lifecycle commands
            api::commands::session::create_session,
            api::commands::session::load_game_into_session,
            api::commands::session::get_session,
            api::commands::session::get_all_sessions,
            api::commands::session::close_session,
            api::commands::session::close_all_sessions,
            // Session operation commands
            api::commands::session::make_move,
            api::commands::session::undo_move,
            api::commands::session::redo_move,
            api::commands::session::next_move,
            api::commands::session::previous_move,
            api::commands::session::navigate_to_end,
            api::commands::session::navigate_to_start,
            api::commands::session::reset_to_position,
            api::commands::session::get_session_moves,
            // Session persistence commands
            api::commands::session::save_session,
            api::commands::session::save_all_sessions,
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
