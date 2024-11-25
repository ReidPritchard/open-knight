use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use diesel::{prelude::Insertable, Associations, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

/**
 * Represents a Game in the chess database.
 *
 * Contains metadata about the game such as the players, event, date, result, etc.
 * The actual moves of the game are stored separately in the `moves` table.
 */
#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Debug, Selectable, Default, Clone, Insertable,
)]
#[diesel(table_name=crate::schema::games, primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    #[diesel(deserialize_as = i32)]
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

/**
 * Represents a Move in the chess database.
 *
 * Each move belongs to a specific game and has a parent and child position.
 * The move is represented in Standard Algebraic Notation (SAN).
 */
#[derive(
    Queryable,
    Identifiable,
    Serialize,
    Deserialize,
    Debug,
    Selectable,
    Default,
    Clone,
    Insertable,
    Associations,
)]
#[diesel(
    table_name=crate::schema::moves,
    belongs_to(Game),
    // belongs_to(Position, foreign_key = parent_position_id),
    // belongs_to(Position, foreign_key = child_position_id),
    check_for_backend(diesel::sqlite::Sqlite),
    primary_key(id)
)]
pub struct Move {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>, // Unique identifier for the move (auto-incremented)
    pub game_id: i32,                 // Foreign key to the game
    pub move_number: i32,             // The move number in the game (half-move number)
    pub move_san: String,             // The move in Standard Algebraic Notation
    pub annotation: Option<String>,   // Comments or annotations for the move
    pub variation_order: Option<i32>, // The order of the move in the variation
    pub parent_position_id: i32,      // Foreign key to the parent position
    pub child_position_id: i32,       // Foreign key to the child position
}

#[derive(
    Queryable, Identifiable, Serialize, Deserialize, Debug, Selectable, Default, Clone, Insertable,
)]
#[diesel(table_name=crate::schema::positions, primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Position {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub fen: String,
    pub annotation: Option<String>,
}
