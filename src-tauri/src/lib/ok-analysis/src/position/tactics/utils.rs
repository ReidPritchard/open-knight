use log::error;
use shakmaty::{attacks, Bitboard, ByColor, ByRole, Color, Role, Square};

use crate::utils::{
    get_bitboard_by_color, get_piece_role_at_square, get_piece_value,
};

/// Checks if a piece is pinned and cannot move without exposing the king
pub fn is_piece_pinned(
    piece_square: Square,
    piece_color: Color,
    all_pieces: Bitboard,
    bitboards_by_role: &ByRole<Bitboard>,
    bitboards_by_color: &ByColor<Bitboard>,
) -> bool {
    // Find the king of the same color
    let king_bitboard = bitboards_by_role.king
        & get_bitboard_by_color(piece_color, bitboards_by_color);
    if let Some(king_square) = king_bitboard.first() {
        // Check if the piece and king are aligned
        if !attacks::aligned(piece_square, king_square, king_square) {
            return false;
        }

        // Get enemy pieces that could create pins
        let enemy_color = !piece_color;
        let enemy_pieces =
            get_bitboard_by_color(enemy_color, bitboards_by_color);
        let enemy_queens = bitboards_by_role.queen & enemy_pieces;
        let enemy_rooks = bitboards_by_role.rook & enemy_pieces;
        let enemy_bishops = bitboards_by_role.bishop & enemy_pieces;

        // Check for pins along ranks/files (rooks and queens)
        for square in enemy_queens | enemy_rooks {
            if attacks::aligned(piece_square, king_square, square) {
                let between = attacks::between(king_square, square);
                let pieces_between = between & all_pieces;
                // If only our piece is between the king and the enemy sliding piece, it's pinned
                if pieces_between == Bitboard::from(piece_square) {
                    return true;
                }
            }
        }

        // Check for pins along diagonals (bishops and queens)
        for square in enemy_queens | enemy_bishops {
            if attacks::aligned(piece_square, king_square, square) {
                let between = attacks::between(king_square, square);
                let pieces_between = between & all_pieces;
                // If only our piece is between the king and the enemy sliding piece, it's pinned
                if pieces_between == Bitboard::from(piece_square) {
                    return true;
                }
            }
        }
    }

    false
}

/// Calculates attacks for a piece on a given square
pub fn calculate_piece_attacks(
    piece_role: Role,
    square: Square,
    all_pieces: Bitboard,
    piece_color: Color,
) -> Bitboard {
    match piece_role {
        Role::Pawn => attacks::pawn_attacks(piece_color, square),
        Role::Knight => attacks::knight_attacks(square),
        Role::Bishop => attacks::bishop_attacks(square, all_pieces),
        Role::Rook => attacks::rook_attacks(square, all_pieces),
        Role::Queen => attacks::queen_attacks(square, all_pieces),
        Role::King => attacks::king_attacks(square),
    }
}

/// Finds all defenders of a given square
///
/// Parameters:
/// - `square`: The square to find defenders for.
/// - `pieces`: A `ByRole` struct containing bitboards for each piece type of the defending player.
/// - `all_pieces`: A bitboard containing all pieces on the board (both colors)
///
/// Returns:
/// - A `ByRole` struct containing bitboards for each piece type of the defending player that can attack the given square.
pub fn find_defenders(
    square: Square,
    side: Color,
    pieces: ByRole<Bitboard>,
    all_pieces: Bitboard,
) -> ByRole<Bitboard> {
    let mut defenders = ByRole::default();

    // Pawns
    for from in pieces.pawn {
        if attacks::pawn_attacks(side, from).contains(square) {
            defenders.pawn |= Bitboard::from(from);
        }
    }
    // Knights
    for from in pieces.knight {
        if attacks::knight_attacks(from).contains(square) {
            defenders.knight |= Bitboard::from(from);
        }
    }
    // Bishops
    for from in pieces.bishop {
        if attacks::bishop_attacks(from, all_pieces).contains(square) {
            defenders.bishop |= Bitboard::from(from);
        }
    }
    // Rooks
    for from in pieces.rook {
        if attacks::rook_attacks(from, all_pieces).contains(square) {
            defenders.rook |= Bitboard::from(from);
        }
    }
    // Queens
    for from in pieces.queen {
        if attacks::queen_attacks(from, all_pieces).contains(square) {
            defenders.queen |= Bitboard::from(from);
        }
    }
    // Kings
    for from in pieces.king {
        if attacks::king_attacks(from).contains(square) {
            defenders.king |= Bitboard::from(from);
        }
    }

    defenders
}
