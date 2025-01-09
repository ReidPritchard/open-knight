use serde::{Deserialize, Serialize};

/// A game in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: Option<i32>,
    pub pgn: String,
    pub player_white: Option<String>,
    pub player_black: Option<String>,
    pub event: Option<String>,
    pub date_text: Option<String>,
    pub result: Option<String>,
    pub annotations: Option<String>,
    pub opening_name: Option<String>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: None,
            pgn: String::new(),
            player_white: None,
            player_black: None,
            event: None,
            date_text: None,
            result: None,
            annotations: None,
            opening_name: None,
        }
    }
}

/// A move in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub id: Option<i32>,
    pub game_id: i32,
    pub move_number: i32,
    pub move_san: String,
    pub annotation: Option<String>,
    pub variation_order: Option<i32>,
    pub parent_position_id: i32,
    pub child_position_id: i32,
}

/// A position in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: Option<i32>,
    pub fen: String,
    pub annotation: Option<String>,
}

/// A header in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub id: Option<i32>,
    pub game_id: i32,
    pub header_key: String,
    pub header_value: String,
}
