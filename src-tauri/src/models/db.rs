use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_bind::TsBind;

/// Core game type that represents a chess game in the database
///
/// TODO: the fields are metadata about the game and ideally
/// cover the 7-archive tags that all games should have.
/// See: http://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c8.1.1
/// All other tags are stored in the `headers` table.
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
    TsBind,
)]
#[ts_bind(export = "../src/shared/bindings")]
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

/// Represents a chess move in the database
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
    TsBind,
)]
#[ts_bind(export = "../src/shared/bindings")]
#[diesel(
    table_name=crate::schema::moves,
    belongs_to(Game),
    check_for_backend(diesel::sqlite::Sqlite),
    primary_key(id)
)]
pub struct Move {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>, // TODO: Auto incremented by database so nullable for insert, but not when queried
    pub game_id: i32,
    pub move_number: i32,
    pub move_san: String,
    pub annotation: Option<String>,
    pub variation_order: Option<i32>,
    pub parent_position_id: i32,
    pub child_position_id: i32,
}

/// Represents a chess position in the database
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
    TsBind,
)]
#[ts_bind(export = "../src/shared/bindings")]
#[diesel(table_name=crate::schema::positions, primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Position {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>, // TODO: Auto incremented by database so nullable for insert, but not when queried
    pub fen: String,
    pub annotation: Option<String>,
}

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
    TsBind,
)]
#[ts_bind(export = "../src/shared/bindings")]
#[diesel(
    table_name=crate::schema::headers,
    primary_key(id),
    belongs_to(Game),
    check_for_backend(diesel::sqlite::Sqlite)
)]
pub struct Header {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>, // TODO: Auto incremented by database so nullable for insert, but not when queried
    pub game_id: i32,
    pub header_key: String,
    pub header_value: String,
}
