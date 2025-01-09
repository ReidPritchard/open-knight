// The chess module is responsible for the chess logic
// anything that modifies a game (moves, annotations, etc)
// Most actual chess-specific logic will be done with the shakmaty library

pub mod tree;

use shakmaty::Chess;

use crate::models::api::ValidMove;
use crate::models::Position;
use crate::models::{api::AllValidMoves, APIGame, APIMove};

#[derive(Debug)]
pub struct ChessNode {
    pub position: Position,
    pub game_move: Option<APIMove>, // Root node will have None
    pub variations: Vec<ChessNode>,
}

impl ChessNode {
    pub fn new(position: Position, game_move: Option<APIMove>) -> Self {
        Self {
            position,
            game_move,
            variations: vec![],
        }
    }

    pub fn default() -> Self {
        Self {
            position: Position::default(),
            game_move: None,
            variations: vec![],
        }
    }
}

/// A struct that represents the current position/state of a game
/// You can think of it as a node in a tree of moves
/// that also contains the overall game_id
#[derive(Debug)]
pub struct ChessGameState {
    pub game_id: i32,
    pub root: ChessNode,
    pub current_node: ChessNode,
}

struct MoveIterator {
    moves: Vec<APIMove>,
    index: usize,
}

impl MoveIterator {
    pub fn new(moves: Vec<APIMove>) -> Self {
        Self { moves, index: 0 }
    }

    pub fn next(&mut self, variation_index: Option<usize>) -> Option<APIMove> {
        let variation_index = variation_index.unwrap_or(0);
        let current_move = self.moves.get(self.index)?;
        // Next position
        let next_position = current_move.child_position.clone();
        // Next move
        let next_move = self.moves.iter().find(|search_move| {
            search_move.parent_position.id == next_position.id
                && search_move.game_move.variation_order == Some(variation_index as i32)
        })?;

        self.index = self
            .moves
            .iter()
            .position(|m| m.game_move.id == next_move.game_move.id)?;

        Some(next_move.clone())
    }

    pub fn previous(&mut self) -> Option<APIMove> {
        let current_move = self.moves.get(self.index)?;
        // Previous position
        let previous_position = current_move.parent_position.clone();
        // Previous move
        let previous_move = self
            .moves
            .iter()
            .find(|search_move| search_move.child_position.id == previous_position.id)?;

        self.index = self
            .moves
            .iter()
            .position(|m| m.game_move.id == previous_move.game_move.id)?;

        Some(previous_move.clone())
    }

    pub fn next_moves(&self) -> Vec<&APIMove> {
        let current_move = self.moves.get(self.index).unwrap();
        let next_moves = self
            .moves
            .iter()
            .filter(|search_move| {
                search_move.parent_position.id == current_move.child_position.id
                    && search_move.game_move.move_number == current_move.game_move.move_number + 1
            })
            .collect();

        next_moves
    }
}

pub fn get_all_valid_moves(position: &str) -> AllValidMoves {
    // Parse the FEN string into a shakmaty Position
    let position_fen = shakmaty::fen::Fen::from_ascii(position.as_bytes()).unwrap();
    let chess_position: shakmaty::Chess = position_fen
        .into_position(shakmaty::CastlingMode::Standard)
        .unwrap();

    // Get all legal moves in this position
    let legal_moves = chess_position.legal_moves();

    // Convert the moves to APIMove format
    let moves = legal_moves
        .iter()
        .map(|mv| {
            // Create a new position after making this move
            let mut new_position = chess_position.clone();
            new_position.play_unchecked(mv);

            ValidMove::from(mv)
        })
        .collect();

    AllValidMoves {
        position: position.to_string(),
        moves,
    }
}
