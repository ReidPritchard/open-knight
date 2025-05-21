use serde::{Deserialize, Serialize};
use shakmaty::san::San;
use shakmaty::{fen::Fen, CastlingMode, Chess, Position};
use ts_rs::TS;

use crate::ts_export;

ts_export! {
    pub struct LegalMove {
        pub san: String,
        pub uci: String,
    }
}

pub fn get_legal_moves(fen: &str) -> Result<Vec<LegalMove>, Box<dyn std::error::Error>> {
    // Parse the FEN string into a Fen struct, then convert to Chess position
    let fen: Fen = fen.parse()?;
    let position: Chess = fen.into_position(CastlingMode::Standard)?;

    // Generate all legal moves in this position
    let legal_moves = position.legal_moves();

    // Convert moves to our LegalMove format
    let moves = legal_moves
        .into_iter()
        .map(|m| LegalMove {
            san: San::from_move(&position, &m).to_string(),
            uci: m.to_uci(shakmaty::CastlingMode::Standard).to_string(),
        })
        .collect();

    Ok(moves)
}
