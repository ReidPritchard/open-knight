use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

use super::db::{Game, Move, Position};
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

/// Represents a game with its moves for API communication
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct APIGame {
    #[serde(flatten)]
    pub game: Game,
    pub moves: Vec<APIMove>,
}

impl From<(Game, Vec<APIMove>)> for APIGame {
    fn from((game, moves): (Game, Vec<APIMove>)) -> Self {
        APIGame { game, moves }
    }
}

impl From<Game> for APIGame {
    fn from(game: Game) -> Self {
        APIGame {
            game,
            moves: Vec::new(),
        }
    }
}

impl From<FullGame> for APIGame {
    fn from(full_game: FullGame) -> Self {
        APIGame {
            game: full_game.game,
            moves: Vec::new(), // Convert moves if needed
        }
    }
}
