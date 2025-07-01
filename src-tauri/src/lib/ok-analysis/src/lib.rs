//! Analysis module
//!
//! Used for analyzing games and positions by extracting metadata and
//! interpreting the engine evaluation. Essentially trying to determine
//! the "why" behind the engine's evaluations/moves.
//!
//! This module is responsible for:
//! - Extracting position/move metadata (tactical, positional, etc.)
//! - Categorizing moves based on move context, metadata, and engine evaluation
//! - Extracting high-level multi-move/game level concepts (e.g. "queenside attack", "bishop pair", etc.)
//! - Interpreting the engine evaluation
//!
//! This module is not responsible for:
//! - Running the engine
//! - Saving analysis results to the database
//! - Providing analysis results to the user

use serde::{Deserialize, Serialize};

pub mod position;
pub mod utils;

// TODO: Organize this module into submodules
// maybe something like:
// - position analysis (e.g. tactical, positional, etc.)
// - move analysis (e.g. move context, metadata, engine evaluation)
// - game analysis (e.g. high-level multi-move/game level patterns)
// - evaluation summary (e.g. summary statistics of a game's analysis)

/// Move category assignment
#[derive(Debug, Clone, serde::Serialize)]
pub enum MoveCategory {
    Book,
    Brilliant,
    Best,
    Excellent,
    Good,
    Inaccuracy,
    Mistake,
    Blunder,
}

/// Summary statistics of a game's analysis
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameAnalysisSummary {
    pub brilliant_moves: u32,
    pub excellent_moves: u32,
    pub good_moves: u32,
    pub inaccuracies: u32,
    pub mistakes: u32,
    pub blunders: u32,
    pub average_centipawn_loss: f32,
}

impl Default for GameAnalysisSummary {
    fn default() -> Self {
        Self {
            brilliant_moves: 0,
            excellent_moves: 0,
            good_moves: 0,
            inaccuracies: 0,
            mistakes: 0,
            blunders: 0,
            average_centipawn_loss: 0.0,
        }
    }
}

/// Configuration for game analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaAnalysisConfig {
    /// Move categorization thresholds (in centipawns)
    pub move_thresholds: MoveCategoryThresholds,
    /// Maximum time to wait for engine analysis per position (seconds)
    pub max_analysis_timeout: u64,
}

/// Move categorization thresholds in centipawns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCategoryThresholds {
    pub brilliant: i32, // Move improving position significantly (e.g., +50 cp)
    pub excellent: i32, // Very good move (e.g., within -10 cp of best)
    pub good: i32,      // Good move (e.g., within -25 cp of best)
    pub inaccuracy: i32, // Minor mistake (e.g., -50 cp from best)
    pub mistake: i32,   // Significant mistake (e.g., -100 cp from best)
    pub blunder: i32,   // Major mistake (e.g., -200+ cp from best)
}

/// Result of analyzing a single position
#[derive(Debug, Clone, Serialize)]
pub struct PositionAnalysisResult {
    pub fen: String,
    pub engine_evaluation: Option<f32>,
    pub best_move: Option<String>,
    pub analysis_time_ms: u64,
}

/// Analysis result for a single move
#[derive(Debug, Clone, Serialize)]
pub struct MoveAnalysisResult {
    pub move_id: i32,
    pub san: String,
    pub uci: String,
    pub ply_number: i32,
    pub position_before: PositionAnalysisResult,
    pub position_after: PositionAnalysisResult,
    pub move_category: Option<MoveCategory>,
    pub evaluation_difference: Option<f32>, // Difference from engine's best move
}

/// Complete game analysis result
#[derive(Debug, Clone, Serialize)]
pub struct GameAnalysisResult {
    pub game_id: i32,
    pub engine_name: String,
    pub analysis_config: MetaAnalysisConfig,
    pub move_analyses: Vec<MoveAnalysisResult>,
    pub total_analysis_time_ms: u64,
    pub positions_analyzed: u32,
    pub evaluation_summary: GameAnalysisSummary,
}

impl Default for MetaAnalysisConfig {
    fn default() -> Self {
        Self {
            move_thresholds: MoveCategoryThresholds::default(),
            max_analysis_timeout: 30,
        }
    }
}

impl Default for MoveCategoryThresholds {
    fn default() -> Self {
        Self {
            brilliant: 50,   // +50 cp improvement or more
            excellent: -10,  // Within 10 cp of best
            good: -25,       // Within 25 cp of best
            inaccuracy: -50, // 50 cp worse than best
            mistake: -100,   // 100 cp worse than best
            blunder: -200,   // 200+ cp worse than best
        }
    }
}
