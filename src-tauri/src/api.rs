use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

use super::db::{Game, Header, Move, Position};
use super::game::FullGame;

/// Represents a move with its parent and child positions.
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct APIMove {
    #[serde(flatten)]
    pub game_move: Move,
    pub parent_position: Position,
    pub child_position: Position,
}

impl From<(Move, Position, Position)> for APIMove {
    fn from((game_move, parent_position, child_position): (Move, Position, Position)) -> Self {
        APIMove {
            game_move,
            parent_position,
            child_position,
        }
    }
}

// Represents all valid moves for a given position
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct AllValidMoves {
    pub position: String,
    pub moves: Vec<ValidMove>,
}

// Represents a valid move for a given position
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct ValidMove {
    pub move_san: String,
    pub from_square: String,
    pub to_square: String,
    // Add any other fields as needed
    // not sure what else would be though
}

impl From<&shakmaty::Move> for ValidMove {
    fn from(_move: &shakmaty::Move) -> Self {
        ValidMove {
            move_san: _move.to_string(),
            from_square: _move.from().unwrap().to_string(),
            to_square: _move.to().to_string(),
        }
    }
}

/// Represents a game with its moves for API communication
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct APIGame {
    #[serde(flatten)]
    pub game: Game,
    pub moves: Vec<APIMove>,
    pub headers: Vec<Header>,
}

impl From<(Game, Vec<APIMove>, Vec<Header>)> for APIGame {
    fn from((game, moves, headers): (Game, Vec<APIMove>, Vec<Header>)) -> Self {
        APIGame {
            game,
            moves,
            headers,
        }
    }
}

impl From<Game> for APIGame {
    fn from(game: Game) -> Self {
        APIGame {
            game,
            moves: Vec::new(),
            headers: Vec::new(),
        }
    }
}

impl From<FullGame> for APIGame {
    fn from(full_game: FullGame) -> Self {
        APIGame {
            game: full_game.game,
            moves: Vec::new(), // Convert moves if needed
            headers: Vec::new(),
        }
    }
}
