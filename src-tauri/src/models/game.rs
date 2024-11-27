use serde::{Deserialize, Serialize};
use shakmaty::Chess;
use ts_bind::TsBind;

use super::db::{Game, Move};

/// Represents a game during PGN parsing
/// Extends Game with parsing-specific fields
#[derive(Debug, Clone)]
pub struct ParsingGame {
    pub game: Game,
    pub chess_position: Option<Chess>,
    pub moves: Vec<Move>,
    pub errors: Vec<String>,
}

impl From<Game> for ParsingGame {
    fn from(game: Game) -> Self {
        ParsingGame {
            game,
            chess_position: Some(Chess::default()),
            moves: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl From<ParsingGame> for Game {
    fn from(parsing_game: ParsingGame) -> Self {
        parsing_game.game
    }
}

/// Represents a game with its moves and positions
/// Used for full game viewing and analysis
#[derive(Debug, Clone, Serialize, Deserialize, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct FullGame {
    pub game: Game,
    pub moves: Vec<Move>,
}

/// Represents a game in the explorer view
/// A lightweight version with just headers for the game list
#[derive(Debug, Clone, Serialize, Deserialize, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct ExplorerGame {
    pub game: Game,
}

impl From<Game> for ExplorerGame {
    fn from(game: Game) -> Self {
        ExplorerGame { game }
    }
}

impl From<FullGame> for ExplorerGame {
    fn from(full_game: FullGame) -> Self {
        ExplorerGame {
            game: full_game.game,
        }
    }
}
