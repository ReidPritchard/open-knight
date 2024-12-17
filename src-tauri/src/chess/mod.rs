// The chess module is responsible for the chess logic
// anything that modifies a game (moves, annotations, etc)
// Most actual chess-specific logic will be done with the shakmaty library

use shakmaty::Chess;

use crate::models::{APIGame, APIMove, Move};

pub struct EditableGame {
    /**
     * A chess board placeholder.
     *
     * This is allocated once and re-used for all operations.
     * It's not guaranteed to be the latest move or variation
     * as it's only updated when a change is made or the UI
     * needs to be updated.
     */
    board: Chess,

    /**
     * The full game object
     *
     * contains, moves, headers, etc
     */
    game: APIGame,

    /**
     * The current move being edited
     */
    current_move: Option<APIMove>,

    /**
     * The current variation being edited
     *
     * This is the index of the current variation in the game
     * This is only used if the current move either has variations
     * or a new variation is created.
     *
     * Essentially this is a "which path/move is next"
     */
    current_variation_index: usize,

    /**
     * The iterator for the current move
     */
    current_move_iterator: MoveIterator,
}

impl EditableGame {
    pub fn new(game: APIGame) -> Self {
        Self {
            board: Chess::new(),
            game: game.clone(),
            current_move: None,
            current_variation_index: 0,
            current_move_iterator: MoveIterator::new(game.moves.clone()),
        }
    }

    pub fn get_current_move(&self) -> APIMove {
        self.current_move.clone().unwrap()
    }

    pub fn next_move(&mut self) {
        self.current_move = self.current_move_iterator.next(None);
    }

    pub fn previous_move(&mut self) {
        self.current_move = self.current_move_iterator.previous();
    }
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
}
