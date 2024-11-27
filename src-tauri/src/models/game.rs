use serde::{Deserialize, Serialize};
use shakmaty::Chess;
use ts_bind::TsBind;

use super::db::{Game, Header, Move};

/// Represents a game during PGN parsing
/// Extends Game with parsing-specific fields
#[derive(Debug, Clone)]
pub struct ParsingGame {
    pub game: Game,
    pub moves: Vec<Move>,
    pub headers: Vec<Header>,
    pub errors: Vec<String>,
    pub chess_position: Option<Chess>,
}

impl From<Game> for ParsingGame {
    fn from(game: Game) -> Self {
        ParsingGame {
            game,
            chess_position: Some(Chess::default()),
            moves: Vec::new(),
            headers: Vec::new(),
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
    pub headers: Vec<Header>,
}

/// Represents a game in the explorer view
/// A lightweight version with just headers for the game list
#[derive(Debug, Clone, Serialize, Deserialize, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct ExplorerGame {
    pub game: Game,
    pub headers: Vec<Header>,
}

impl From<(Game, Vec<Header>)> for ExplorerGame {
    fn from((game, headers): (Game, Vec<Header>)) -> Self {
        ExplorerGame { game, headers }
    }
}

impl From<FullGame> for ExplorerGame {
    fn from(full_game: FullGame) -> Self {
        ExplorerGame {
            game: full_game.game,
            headers: full_game.headers,
        }
    }
}
