use crate::models::{Game, Move, Position};
use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

////////////// API Types //////////////
// These types are used for interacting with the frontend
// as well as most logic in the backend aside from the database.
// Most are combinations of the database types either created by joins or other logic.

/**
 * Represents a move with its parent and child positions.
 * All properties from the Move type, plus the parent and child positions (rather than ids)
 */
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

/**
 * Represents a game with its id, headers, and moves.
 *
 * Similar to GameResult, but simplified as there is no validation of the game.
 */
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct APIGame {
    #[serde(flatten)]
    pub game_data: Game,
    pub moves: Vec<APIMove>,
}

impl From<(Game, Vec<APIMove>)> for APIGame {
    fn from((game, moves): (Game, Vec<APIMove>)) -> Self {
        APIGame {
            game_data: game,
            moves,
        }
    }
}

/// A type for (string, string) tuples so the bindings can be generated correctly
///
///
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct HeaderTuple {
    pub key: String,
    pub value: String,
}

impl From<(String, String)> for HeaderTuple {
    fn from((key, value): (String, String)) -> Self {
        HeaderTuple { key, value }
    }
}

/**
 * Represents a game in the explorer state.
 *
 * This is really just the headers and the id of the game.
 * It is used to represent a game in the explorer state.
 */
#[derive(Serialize, Deserialize, Debug, Clone, TsBind)]
#[ts_bind(export = "../src/shared/bindings")]
pub struct ExplorerGame {
    pub id: i32,
    pub headers: Vec<HeaderTuple>,
}

impl From<Game> for ExplorerGame {
    fn from(game: Game) -> Self {
        let headers = vec![
            ("Event".to_string(), game.event.unwrap_or("".to_string())),
            ("Date".to_string(), game.date_text.unwrap_or("".to_string())),
            ("Result".to_string(), game.result.unwrap_or("".to_string())),
            (
                "White".to_string(),
                game.player_white.unwrap_or("".to_string()),
            ),
            (
                "Black".to_string(),
                game.player_black.unwrap_or("".to_string()),
            ),
            (
                "Opening".to_string(),
                game.opening_name.unwrap_or("".to_string()),
            ),
            (
                "Annotations".to_string(),
                game.annotations.unwrap_or("".to_string()),
            ),
        ];

        ExplorerGame {
            id: game.id.unwrap_or(0),
            headers: headers.into_iter().map(HeaderTuple::from).collect(),
        }
    }
}

impl From<APIGame> for ExplorerGame {
    fn from(api_game: APIGame) -> Self {
        ExplorerGame::from(api_game.game_data)
    }
}
