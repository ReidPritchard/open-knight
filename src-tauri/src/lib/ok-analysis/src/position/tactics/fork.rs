use super::utils::{calculate_piece_attacks, find_defenders, is_piece_pinned};
use crate::position::TacticalPattern;
use crate::utils::{
    get_bitboard_by_color, get_bitboard_by_role, get_piece_role_at_square,
    get_piece_value,
};
use log::{debug, error, info};
use shakmaty::{Bitboard, ByColor, ByRole, Color, Role, Square};

/// Detects all forks for the given side
///
/// TODO: refactor this function
pub fn detect_forks(
    attacking_color: Color,
    bitboards_by_role: &ByRole<Bitboard>,
    bitboards_by_color: &ByColor<Bitboard>,
) -> Vec<TacticalPattern> {
    let mut forks = Vec::new();
    let all_pieces = get_bitboard_by_color(Color::White, bitboards_by_color)
        | get_bitboard_by_color(Color::Black, bitboards_by_color);
    let attacking_pieces =
        get_bitboard_by_color(attacking_color, bitboards_by_color);
    let defending_pieces =
        get_bitboard_by_color(!attacking_color, bitboards_by_color);

    // Check each piece type for potential forks
    let piece_types = [
        Role::Pawn,
        Role::Knight,
        Role::Bishop,
        Role::Rook,
        Role::Queen,
        Role::King,
    ];

    for piece_type in piece_types {
        let pieces_of_type =
            get_bitboard_by_role(piece_type, bitboards_by_role)
                & attacking_pieces;

        for piece_square in pieces_of_type {
            // Skip if the piece is pinned (cannot move without exposing king)
            if is_piece_pinned(
                piece_square,
                attacking_color,
                all_pieces,
                bitboards_by_role,
                bitboards_by_color,
            ) {
                debug!(
                    "Piece at {} is pinned, skipping fork analysis",
                    piece_square
                );
                println!(
                    "Piece at {} is pinned, skipping fork analysis",
                    piece_square
                );
                continue;
            }

            // Calculate what this piece attacks
            let piece_attacks = calculate_piece_attacks(
                piece_type,
                piece_square,
                all_pieces,
                attacking_color,
            );

            // Find enemy pieces being attacked
            let mut attacked_enemy_squares = piece_attacks & defending_pieces;

            // Check if any of the attacked pieces are defended
            let defenders_by_role = find_defenders(
                piece_square,
                !attacking_color,
                *bitboards_by_role,
                all_pieces,
            );
            // Remove any defended squares from the attacked squares
            attacked_enemy_squares &= defenders_by_role.pawn
                & defenders_by_role.knight
                & defenders_by_role.bishop
                & defenders_by_role.rook
                & defenders_by_role.queen;

            if attacked_enemy_squares.more_than_one() {
                // Identify the roles of attacked pieces
                let mut attacked_pieces = Vec::new();
                for attacked_square in attacked_enemy_squares {
                    if let Some(role) = get_piece_role_at_square(
                        attacked_square,
                        bitboards_by_role,
                    ) {
                        attacked_pieces.push((attacked_square, role));
                    }
                }

                // Check if this is a meaningful fork
                if is_meaningful_fork(piece_type, &attacked_pieces) {
                    let attacked_piece_names: Vec<String> = attacked_pieces
                        .iter()
                        .map(|(sq, role)| format!("{:?} on {}", role, sq))
                        .collect();

                    info!(
                        "{:?} fork found: {:?} on {} attacks {}",
                        piece_type,
                        piece_type,
                        piece_square,
                        attacked_piece_names.join(", ")
                    );
                    println!(
                        "{:?} fork found: {:?} on {} attacks {}",
                        piece_type,
                        piece_type,
                        piece_square,
                        attacked_piece_names.join(", ")
                    );

                    forks.push(TacticalPattern {
                        name: crate::position::TacticalPatternType::Fork,
                    });
                } else {
                    let attacked_piece_names: Vec<String> = attacked_pieces
                        .iter()
                        .map(|(sq, role)| format!("{:?} on {}", role, sq))
                        .collect();
                    // No meaningful fork found
                    // log the fork, but say it was not meaningful
                    debug!(
                        "Meaningless {:?} fork found: {:?} on {} attacks {}",
                        piece_type,
                        piece_type,
                        piece_square,
                        attacked_piece_names.join(", ")
                    );
                    println!(
                        "Meaningless {:?} fork found: {:?} on {} attacks {}",
                        piece_type,
                        piece_type,
                        piece_square,
                        attacked_piece_names.join(", ")
                    );
                }
            }
        }
    }

    forks
}

/// Evaluates if a fork is tactically meaningful
pub fn is_meaningful_fork(
    attacking_piece_role: Role,
    attacked_pieces: &[(Square, Role)],
) -> bool {
    if attacked_pieces.len() < 2 {
        error!("Invalid fork provided");
        return false;
    }

    // TODO: Improve the requirements for meaningful forks
    // TODO: We should also consider if the attacked pieces are protected when evaluating
    // their values. Ex. an undefended pawn is still valuable/worth forking,
    // but a protected pawn is not.
    // TODO: Consider if the attacker is protected/safe or if it can be captured
    // causing the fork to be meaningless

    let attacker_value = get_piece_value(attacking_piece_role);

    let valuable_targets = attacked_pieces
        .iter()
        .filter(|(_, role)| {
            *role == Role::King || get_piece_value(*role) >= attacker_value
        })
        .count();

    // Fork is meaningful if it attacks the king or multiple valuable pieces
    attacked_pieces.iter().any(|(_, role)| *role == Role::King)
        || valuable_targets >= 2
}
