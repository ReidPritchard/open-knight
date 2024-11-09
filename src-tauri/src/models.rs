use diesel::{prelude::Insertable, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/**
 * Represents a Game in the chess database.
 */
#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Debug, Selectable, Default, Clone, Insertable,
)]
#[diesel(table_name=crate::schema::games, primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: i32,
    pub pgn: String,
    pub player_white: Option<String>,
    pub player_black: Option<String>,
    pub event: Option<String>,
    pub date: Option<String>,
    pub result: Option<String>,
    pub annotations: Option<String>,
    pub opening_name: Option<String>,
}

/**
 * Represents a Move in the chess database.
 */
#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Debug, Selectable, Default, Clone, Insertable,
)]
#[diesel(table_name=crate::schema::moves, primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Move {
    pub id: i32,                         // Unique identifier for the move (auto-incremented)
    pub game_id: i32,                    // Foreign key to the game
    pub move_number: i32,                // The move number in the game (half-move number)
    pub move_san: String,                // The move in Standard Algebraic Notation
    pub variation_order: Option<i32>,    // The order of the move in the variation
    pub parent_position_id: Option<i32>, // Foreign key to the parent position
    pub child_position_id: Option<i32>,  // Foreign key to the child position
    pub annotation: Option<String>,      // Comments or annotations for the move
}

#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Debug, Selectable, Default, Clone, Insertable,
)]
#[diesel(table_name=crate::schema::positions, primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Position {
    pub id: i32,
    pub fen: String,
    pub annotation: Option<String>,
}
