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
    pub id: i32,
    pub game_id: i32,
    pub move_number: i32,
    pub move_san: String,
    pub variation_id: Option<i32>,
    pub parent_variation_id: Option<i32>,
    pub fen: Option<String>,
    pub annotation: Option<String>,
}
