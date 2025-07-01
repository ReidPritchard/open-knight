//! Determine the "why" behind the engine's evaluation of a position

use log::error;
use shakmaty::{Move, Position};
use tactics::analyze_tactics;

use crate::GameAnalysisSummary;

pub mod tactics;

/// The overall analysis of a position
pub struct PositionAnalysis {
    pub tactical_analysis: Vec<TacticalPattern>,
    pub positional_analysis: Vec<PositionalPattern>,
    pub analysis_summary: GameAnalysisSummary,
}

pub enum TacticalPatternType {
    HangingPiece,
    Fork,
    Pin,
    Skewer,
    XRay,
    Desperado,
    InBetweenMove,
    Attraction,
    Clearance,
    Discovery,
    DoubleCheck,
    Overload,
}

/// A pattern in a position that is considered "tactical"
pub struct TacticalPattern {
    pub name: TacticalPatternType, // "fork", "pin", "skewer", "x-ray", etc.
                                   // TODO: Figure out a common struct that can be used express all tactics
}

/// A positional pattern
pub struct PositionalPattern {
    pub name: String,
    pub description: String,
    // pub included_squares: Vec<String>,
}

/// Given a position, determine the "why" behind the engine's evaluation
///
/// Parameters:
/// - `position`: The position to analyze
/// - `potential_variations`: The potential variations of the position
///
/// Returns:
/// - `position_analysis`: The analysis of the position
pub fn analyze_position<T: Position>(
    position: T,
    potential_variations: Vec<Vec<Move>>,
) -> PositionAnalysis {
    let turn = position.turn();
    let position_board = position.board();
    let position_bitboards = position_board.clone().into_bitboards();

    let tactical_analysis_result =
        analyze_tactics(turn, position_bitboards, &potential_variations);

    let tactical_analysis = match tactical_analysis_result {
        Ok(tactical_analysis) => tactical_analysis,
        Err(error) => {
            error!("Error analyzing tactics: {}", error);
            Vec::new()
        }
    };

    // let positional_analysis = analyze_positionalpatterns(position);

    // let analysis_summary = summarize_analysis(tactical_analysis, positional_analysis);

    PositionAnalysis {
        tactical_analysis,
        positional_analysis: Vec::new(),
        analysis_summary: GameAnalysisSummary::default(),
    }
}
