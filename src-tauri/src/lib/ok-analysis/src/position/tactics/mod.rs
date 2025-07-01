mod fork;
mod utils;

use crate::utils::AnalysisError;
pub use fork::detect_forks;
use shakmaty::{Bitboard, ByColor, ByRole, Color, Move};

use super::TacticalPattern;

/// Given a set of bitboards representing a position and a list of pv lines
/// (determined by the engine), return a list of tactics
pub fn analyze_tactics(
    turn: Color,
    bitboards: (ByRole<Bitboard>, ByColor<Bitboard>),
    _potential_variations: &[Vec<Move>],
) -> Result<Vec<TacticalPattern>, AnalysisError> {
    let bitboards_by_role = bitboards.0;
    let bitboards_by_color = bitboards.1;

    let mut tactics = Vec::new();

    // Analyze forks for the current player
    let current_player_forks =
        fork::detect_forks(turn, &bitboards_by_role, &bitboards_by_color);
    tactics.extend(current_player_forks);

    // Optionally, also analyze opponent forks (threats to be aware of)
    let opponent_forks =
        fork::detect_forks(!turn, &bitboards_by_role, &bitboards_by_color);
    tactics.extend(opponent_forks);

    Ok(tactics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::{fen::Fen, CastlingMode, Chess, Position};

    fn setup_args(
        position_fen: Option<Fen>
    ) -> (Color, (ByRole<Bitboard>, ByColor<Bitboard>), Vec<Vec<Move>>) {
        let pos = position_fen.clone().unwrap_or_else(|| Fen::default());
        let position = pos
            .into_position(CastlingMode::Standard)
            .unwrap_or_else(|_| Chess::default());

        let starting_board = position.board();

        let turn = position.turn();
        let bitboards = starting_board.clone().into_bitboards();
        let potential_variations = &vec![];

        (turn, bitboards, potential_variations.to_vec())
    }

    #[test]
    fn no_tactics() {
        let (turn, bitboards, potential_variations) = setup_args(None);

        let tactics =
            super::analyze_tactics(turn, bitboards, &potential_variations)
                .unwrap();

        assert!(tactics.is_empty());
    }

    #[test]
    fn knight_fork() {
        // Knight on d5 forks rook on c7 and queen on f6
        let fork_fen: Fen =
            Fen::from_ascii(b"4k3/2r5/5q2/3N4/8/8/8/4K3 w - - 0 1").unwrap();
        let (turn, bitboards, potential_variations) =
            setup_args(Some(fork_fen.into()));

        let tactics =
            super::analyze_tactics(turn, bitboards, &potential_variations)
                .unwrap();

        assert!(tactics.len() >= 1, "Should detect at least one fork");
    }

    #[test]
    fn meaningless_fork_low_value() {
        // Queen forks two pawns - not meaningful since queen is more valuable
        let fork_fen: Fen =
            Fen::from_ascii(b"4k3/8/2p1p3/3Q4/8/8/8/4K3 w - - 0 1").unwrap();
        let (turn, bitboards, potential_variations) =
            setup_args(Some(fork_fen.into()));

        let tactics =
            super::analyze_tactics(turn, bitboards, &potential_variations)
                .unwrap();

        // Should not detect this as a meaningful fork
        assert!(
            tactics.is_empty() || tactics.len() == 0,
            "Should not detect meaningless forks"
        );
    }

    #[test]
    fn fork_with_king() {
        // Knight forks king and rook - always meaningful
        let fork_fen: Fen =
            Fen::from_ascii(b"8/8/1r3k2/3N4/8/8/8/4K3 b - - 0 1").unwrap();
        let (turn, bitboards, potential_variations) =
            setup_args(Some(fork_fen.into()));

        let tactics =
            super::analyze_tactics(turn, bitboards, &potential_variations)
                .unwrap();

        assert!(tactics.len() >= 1, "Should detect fork involving king");
    }
}
