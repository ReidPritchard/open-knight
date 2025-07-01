use std::time::Duration;

use super::AppState;
use crate::utils::AppError;
use log::{debug, error, info};
use ok_analysis::*;
use ok_engine_manager::{
    events::EngineStateInfoEvent, manager::EngineAnalysisConfig,
    protocol::OptionValue, utils::calculate_analysis_time,
};
use tauri::State;
use tokio::time::sleep;

/// Gets the state of all loaded chess engines
///
/// Returns a JSON string containing the state of all engines.
#[tauri::command]
pub async fn get_all_engine_state(
    state: State<'_, AppState>
) -> Result<String, String> {
    let engine_manager = state.engine_manager.lock().await;
    let states = engine_manager.get_all_engine_state().await;
    Ok(serde_json::to_string(&states).unwrap())
}

/// Loads a UCI chess engine
///
/// Parameters:
/// - `name`: A unique name for the engine
/// - `path`: The file path to the engine executable
///
/// The engine will be initialized and made available for analysis.
#[tauri::command]
pub async fn load_engine(
    name: String,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    debug!("Loading engine: {}", name);

    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.add_uci_engine(&name, &path).await;
    drop(engine_manager);

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error loading engine: {}", e);
            Err(e.to_string())
        }
    }
}

/// Unloads a previously loaded chess engine
///
/// Parameters:
/// - `name`: The name of the engine to unload
#[tauri::command]
pub async fn unload_engine(
    name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.remove_engine(&name).await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Analyzes a chess position with the loaded engine
///
/// Parameters:
/// - `fen`: The FEN string representing the position to analyze
/// - `depth`: Optional depth limit for the analysis
/// - `time_ms`: Optional time limit in milliseconds
///
/// The analysis results will be sent through event listeners.
///
/// @deprecated: Use
#[tauri::command]
pub async fn analyze_position(
    fen: String,
    depth: Option<usize>,
    time_ms: Option<usize>,
    multipv: Option<u32>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    debug!("Analyzing position");
    let mut engine_manager = state.engine_manager.lock().await;

    engine_manager
        .set_position(Some(&fen), None)
        .await
        .map_err(|e| e.to_string())?;

    let result = engine_manager
        .start_position_analysis(
            depth.map(|d| d as u32),
            time_ms.map(|t| t as u32),
            Some(multipv.unwrap_or(1)),
        )
        .await;

    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Analyzes a chess position derived from a game session's current move/position
///
/// Parameters:
/// - `board_id`: The ID of the board to analyze
/// - `depth`: Optional depth limit for the analysis
/// - `time_ms`: Optional time limit in milliseconds
///
/// The analysis results will be sent through event listeners.
#[tauri::command]
pub async fn analyze_move(
    board_id: i32,
    depth: Option<usize>,
    time_ms: Option<usize>,
    multipv: Option<u32>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // First get the game session
    let game_session_manager = state.game_session_manager.lock().await;
    let game_session = game_session_manager
        .get_session(board_id)
        .ok_or(AppError::SessionError("Game session not found".to_string()))
        .unwrap();

    // Then get the move from the game session
    let current_move_id = game_session.game.move_tree.current_node_id;
    let current_node =
        game_session.game.move_tree.nodes[current_move_id.unwrap()].clone();

    // Get the position from the current node
    let current_position = current_node.position;
    let fen = current_position.fen;

    // Set the position for the engine
    let mut engine_manager = state.engine_manager.lock().await;
    engine_manager
        .set_position(Some(&fen), None)
        .await
        .map_err(|e| e.to_string())?;

    // Start the analysis
    let result = engine_manager
        .start_position_analysis(
            depth.map(|d| d as u32),
            time_ms.map(|t| t as u32),
            Some(multipv.unwrap_or(1)),
        )
        .await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())

    // TODO: We need a way to store the evaluation results on the game move
    // I'm not sure if we should do this here/when the analysis events are emitted
    // or if we should let the client decide if they want to save the results
    // after they receive the events.
}

/// Analyzes a complete chess game from a game session
///
/// Parameters:
/// - `board_id`: The ID of the board to analyze
/// - `include_variations`: Whether to include variations in the analysis
/// - `analysis_config`: Configuration for time management and categorization
///
/// Returns comprehensive analysis results with move categorizations and evaluations
#[tauri::command]
pub async fn analyze_game(
    board_id: i32,
    engine_analysis_config: Option<EngineAnalysisConfig>,
    meta_analysis_config: Option<MetaAnalysisConfig>,
    state: State<'_, AppState>,
) -> Result<GameAnalysisResult, String> {
    let _meta_config = meta_analysis_config.unwrap_or_default();
    let engine_config = engine_analysis_config.unwrap_or_default();

    info!("Starting game analysis for board {}", board_id);
    let _start_time = std::time::Instant::now();

    // 1. Extract positions for analysis
    let game_session_manager = state.game_session_manager.lock().await;
    let game_session = game_session_manager
        .get_session(board_id)
        .ok_or(AppError::SessionError("Game session not found".to_string()))
        .unwrap();
    let positions =
        game_session.extract_positions(engine_config.include_variations);

    info!("Extracted {} positions for analysis", positions.len());

    if positions.is_empty() {
        return Err("No positions found to analyze".to_string());
    }

    // 2. Calculate time per position
    let (depth, time_ms) =
        calculate_analysis_time(&engine_config.time_strategy, positions.len());
    info!(
        "Analysis time per position: depth={:?}, time_ms={:?}",
        depth, time_ms
    );

    // 3. Get engine
    let engine_manager = state.engine_manager.lock().await;
    let engine_option = engine_manager.get_engine(&engine_config.engine_name);
    let engine = engine_option
        // FIXME: Handle this error
        // .ok_or(AppError::EngineError("Engine not found".to_string()))
        .unwrap_or_else(|| panic!("Engine not found"));

    // 4. Analyze each position
    let total_positions = positions.len();
    let mut move_analyses = Vec::new();
    let mut total_positions_analyzed = 0u32;

    for current_position in positions {
        // Log a progress update every 10 positions
        if total_positions_analyzed % 10 == 0 {
            info!(
                "Analyzing move {}/{}",
                total_positions_analyzed + 1,
                total_positions
            );
        }

        let current_fen = &current_position.fen;

        // Start the analysis
        let mut engine_manager = state.engine_manager.lock().await;
        let _ = engine_manager
            .quick_start_position_analysis_for(
                &engine_config.engine_name,
                current_fen,
                depth,
                time_ms,
                None,
            )
            .await;
        drop(engine_manager);

        let mut current_analysis = None;
        // Monitor the engine's updates
        let _ = engine
            .monitor_events(|event| match event {
                EngineStateInfoEvent::AnalysisUpdate(update) => {
                    info!("Analysis update received");
                    current_analysis = Some(update);
                    return true; // Continue monitoring
                }
                EngineStateInfoEvent::BestMove(best_move, best_move_score) => {
                    // Indicates the analysis is complete
                    info!(
                        "Best move received {} score: {:?}",
                        best_move, best_move_score
                    );
                    return false; // Stop monitoring
                }
                _ => {
                    // For any other events, ignore them and continue to monitor
                    return true;
                }
            })
            .await;

        // Current analysis should contain the latest engine evaluation
        // track it
        move_analyses.push(current_analysis);
        total_positions_analyzed += 1;

        // Brief pause to avoid overwhelming the engine
        sleep(Duration::from_millis(100)).await;
    }

    //

    // // 5. Generate summary statistics
    // let mut summary = EvaluationSummary {
    //     brilliant_moves: 0,
    //     excellent_moves: 0,
    //     good_moves: 0,
    //     inaccuracies: 0,
    //     mistakes: 0,
    //     blunders: 0,
    //     average_centipawn_loss: 0.0,
    // };

    // let mut total_centipawn_loss = 0.0;
    // let mut evaluated_moves = 0;

    // for analysis in &move_analyses {
    //     if let Some(category) = &analysis.move_category {
    //         match category {
    //             MoveCategory::Brilliant => summary.brilliant_moves += 1,
    //             MoveCategory::Best => summary.excellent_moves += 1,
    //             MoveCategory::Excellent => summary.excellent_moves += 1,
    //             MoveCategory::Good => summary.good_moves += 1,
    //             MoveCategory::Inaccuracy => summary.inaccuracies += 1,
    //             MoveCategory::Mistake => summary.mistakes += 1,
    //             MoveCategory::Blunder => summary.blunders += 1,
    //         }
    //     }

    //     if let Some(eval_diff) = analysis.evaluation_difference {
    //         if eval_diff < 0.0 {
    //             total_centipawn_loss += eval_diff.abs();
    //             evaluated_moves += 1;
    //         }
    //     }
    // }

    // summary.average_centipawn_loss = if evaluated_moves > 0 {
    //     total_centipawn_loss / evaluated_moves as f32
    // } else {
    //     0.0
    // };

    // let total_time_ms = start_time.elapsed().as_millis() as u64;

    // info!(
    //     "Game analysis completed: {} moves analyzed in {}ms",
    //     move_analyses.len(),
    //     total_time_ms
    // );

    // // Get game ID from session
    // let game_session_manager = state.game_session_manager.lock().await;
    // let game_session =
    //     game_session_manager.get_session(board_id).ok_or_else(|| {
    //         format!("Game session not found for board {}", board_id)
    //     })?;
    // let game_id = game_session.game.id;
    // drop(game_session_manager);

    // Ok(GameAnalysisResult {
    //     game_id,
    //     engine_name,
    //     analysis_config: config,
    //     move_analyses,
    //     total_analysis_time_ms: total_time_ms,
    //     positions_analyzed: total_positions_analyzed,
    //     evaluation_summary: summary,
    // })

    todo!("Implement game analysis")
}

/// Stops any ongoing analysis
///
/// Terminates the current engine analysis and returns the engine to idle state.
#[tauri::command]
pub async fn stop_analysis(state: State<'_, AppState>) -> Result<(), String> {
    debug!("Stopping analysis");
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.stop_analysis().await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Sets an option for a specific chess engine
///
/// Parameters:
/// - `engine_name`: The name of the engine
/// - `option`: The name of the option to set
/// - `value`: The value to set for the option
#[tauri::command]
pub async fn set_engine_option(
    engine_name: String,
    option: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager
        .set_engine_option(&engine_name, &option, OptionValue::String(value))
        .await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}

/// Sets the current position for the engine
///
/// Parameters:
/// - `fen`: The FEN string representing the position to set
#[tauri::command]
pub async fn set_position(
    fen: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut engine_manager = state.engine_manager.lock().await;
    let result = engine_manager.set_position(Some(&fen), None).await;
    drop(engine_manager);
    result.map_err(|e| e.to_string())
}
