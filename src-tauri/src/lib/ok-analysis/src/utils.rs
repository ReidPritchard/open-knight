//! Utility functions and common types

use serde::Serialize;
use shakmaty::{Bitboard, ByColor, ByRole, Color, File, Rank, Role, Square};

/// Error types for analysis
#[derive(Debug, Clone, Serialize, thiserror::Error)]
pub enum AnalysisError {
    /// Error occured due to invalid input
    #[error("Invalid input")]
    InvalidInput,
}

/// Convert a Bitboard into a string representation
pub fn bitboard_to_string(bitboard: Bitboard) -> String {
    let mut result = String::new();
    for rank in Rank::ALL {
        for file in File::ALL {
            if bitboard.contains(Square::from_coords(file, rank)) {
                result.push('X');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

/// Returns white or black pieces based on the color
pub fn get_bitboard_by_color(
    color: Color,
    bitboards_by_color: &ByColor<Bitboard>,
) -> Bitboard {
    if color == Color::White {
        bitboards_by_color.white
    } else {
        bitboards_by_color.black
    }
}

/// Returns the bitboard of a given piece role
pub fn get_bitboard_by_role(
    role: Role,
    bitboards_by_role: &ByRole<Bitboard>,
) -> Bitboard {
    match role {
        Role::Pawn => bitboards_by_role.pawn,
        Role::Knight => bitboards_by_role.knight,
        Role::Bishop => bitboards_by_role.bishop,
        Role::Rook => bitboards_by_role.rook,
        Role::Queen => bitboards_by_role.queen,
        Role::King => bitboards_by_role.king,
    }
}

/// Returns the piece value for material evaluation
pub fn get_piece_value(role: Role) -> u32 {
    match role {
        Role::Pawn => 1,
        Role::Knight => 3,
        Role::Bishop => 3,
        Role::Rook => 5,
        Role::Queen => 9,
        Role::King => 100, // Very high value, but not infinite for comparison purposes
    }
}

/// Returns the role of a piece on a given square, if any
pub fn get_piece_role_at_square(
    square: Square,
    bitboards_by_role: &ByRole<Bitboard>,
) -> Option<Role> {
    if bitboards_by_role.pawn.contains(square) {
        Some(Role::Pawn)
    } else if bitboards_by_role.knight.contains(square) {
        Some(Role::Knight)
    } else if bitboards_by_role.bishop.contains(square) {
        Some(Role::Bishop)
    } else if bitboards_by_role.rook.contains(square) {
        Some(Role::Rook)
    } else if bitboards_by_role.queen.contains(square) {
        Some(Role::Queen)
    } else if bitboards_by_role.king.contains(square) {
        Some(Role::King)
    } else {
        None
    }
}
