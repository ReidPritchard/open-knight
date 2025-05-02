// src-tauri/src/engine/manager.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Listener};

use crate::engine::engine::EngineError;
use crate::engine::uci::UciEngine;
use crate::models::ChessGame;

use super::uci::UciEngineState;

pub struct EngineManager {
    engines: HashMap<String, Arc<Mutex<UciEngine>>>,
    app_handle: AppHandle,
    timeout: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AnalysisResult {
    ply: usize,
    fen: String,
    score: Option<f64>,
    best_moves: Vec<String>,
    category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GameAnalysisResults {
    moves: Vec<AnalysisResult>,
    accuracy: f64,
    engine: String,
}

impl EngineManager {
    pub fn new(app_handle: AppHandle, timeout: Option<u64>) -> Self {
        let timeout = timeout.unwrap_or(10);
        Self {
            engines: HashMap::new(),
            app_handle,
            timeout,
        }
    }

    pub fn load_engine(&mut self, name: String, path: PathBuf) -> Result<(), EngineError> {
        if self.engines.contains_key(&name) {
            return Err(EngineError::EngineAlreadyLoaded(name));
        }

        match UciEngine::new(path, self.app_handle.clone(), Some(self.timeout)) {
            Ok(engine) => {
                self.engines.insert(name, Arc::new(Mutex::new(engine)));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_engine(&self, name: &str) -> Option<Arc<Mutex<UciEngine>>> {
        self.engines.get(name).cloned()
    }

    pub fn unload_engine(&mut self, name: &str) -> Result<(), EngineError> {
        if let Some(engine) = self.engines.remove(name) {
            // Make sure to stop the engine before dropping it
            if let Ok(mut engine) = engine.lock() {
                engine
                    .quit()
                    .map_err(|e| EngineError::EngineProcessError(e.to_string()))?;
            }
            Ok(())
        } else {
            Err(EngineError::EngineNotFound(name.to_string()))
        }
    }

    pub fn list_engines(&self) -> Vec<String> {
        self.engines.keys().cloned().collect()
    }

    pub fn analyze_game(&self, name: &str, game: &mut ChessGame) -> Result<(), EngineError> {
        println!("Analyzing game '{}'", game.id);
        let engine = self
            .get_engine(name)
            .ok_or(EngineError::EngineNotFound(name.to_string()))?;
        let mut engine = engine.lock().unwrap();

        // Extract all positions from the game
        let mut positions = Vec::new();
        let mut current_fen = game.fen.clone();
        positions.push(current_fen.clone());

        game.move_tree.move_to_root();
        for move_node in &game.move_tree.main_line() {
            current_fen = Some(move_node.position.fen.clone());
            positions.push(current_fen);
        }

        let num_positions = positions.len();
        println!("Analyzing {} positions", num_positions);
        // Analyze each position
        let mut results = Vec::new();
        for (i, position_fen) in positions.into_iter().enumerate() {
            println!("Analyzing position {}/{}", i + 1, num_positions);
            // If the fen is None, skip the position
            if position_fen.is_none() {
                // Really should never happen, but just in case
                eprintln!("FEN is None for position {}", i);
                continue;
            }

            let fen = position_fen.unwrap();

            // Set the position
            engine.position_from_fen(&fen)?;

            // Wait for analysis to complete
            let (tx, rx) = std::sync::mpsc::channel();
            let app_handle = self.app_handle.clone();

            // Listen for analysis completion
            let _unlisten = self
                .app_handle
                .listen("engine-analysis-complete", move |event| {
                    println!("Engine analysis complete");
                    let payload = event.payload();
                    if let Ok(state) = serde_json::from_str::<UciEngineState>(payload) {
                        // Categorize the move based on score
                        // TODO: Use the previous position score to determine the change in score given the move
                        let move_category = if let Some(score) = state.current_position_score {
                            if score.abs() > 3.0 {
                                "blunder"
                            } else if score.abs() > 1.5 {
                                "mistake"
                            } else if score.abs() > 0.5 {
                                "inaccuracy"
                            } else {
                                "good"
                            }
                        } else {
                            "unknown"
                        };

                        // Create analysis result
                        let result = AnalysisResult {
                            ply: i,
                            fen: fen.clone(),
                            score: state.current_position_score,
                            best_moves: state.current_position_lines,
                            category: move_category.to_string(),
                        };

                        // Send result to channel
                        let _ = tx.send(result.clone());

                        // Emit to frontend
                        let _ = app_handle.emit(
                            "game-analysis-progress",
                            serde_json::to_string(&result).unwrap(),
                        );
                    }
                });

            // TODO: Use the config to determine the depth
            // Start analysis with a reasonable depth
            engine.go_depth(5)?;

            // Wait for analysis result
            if let Ok(result) = rx.recv_timeout(std::time::Duration::from_secs(10)) {
                results.push(result);

                // Stop current analysis
                engine.stop()?;
            }
        }

        println!("Game analysis complete");

        // Calculate overall accuracy
        let total_moves = results.len();
        let good_moves = results.iter().filter(|r| r.category == "good").count();
        let accuracy = (good_moves as f64 / total_moves as f64) * 100.0;

        // Emit final results
        let final_results = GameAnalysisResults {
            moves: results,
            accuracy,
            engine: name.to_string(),
        };

        if let Err(e) = self.app_handle.emit(
            "game-analysis-complete",
            serde_json::to_string(&final_results).unwrap(),
        ) {
            eprintln!("Failed to emit game analysis results: {}", e);
        }

        Ok(())
    }
}
