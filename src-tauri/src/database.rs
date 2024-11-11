use diesel::{prelude::*, query_builder::QueryId};
use dotenvy::dotenv;
use std::env;

use crate::models::{Game, Move, Position};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn empty_db() {
    let mut conn = establish_connection();
    diesel::delete(crate::schema::games::table)
        .execute(&mut conn)
        .unwrap();
    diesel::delete(crate::schema::moves::table)
        .execute(&mut conn)
        .unwrap();
}

pub fn get_game_id_count() -> i64 {
    let mut conn = establish_connection();
    crate::schema::games::table
        .count()
        .get_result(&mut conn)
        .unwrap()
}

pub fn get_all_games() -> Vec<Game> {
    let mut conn = establish_connection();
    crate::schema::games::table.load(&mut conn).unwrap()
}

pub fn get_all_moves() -> Vec<Move> {
    let mut conn = establish_connection();
    crate::schema::moves::table.load(&mut conn).unwrap()
}

pub fn get_move_id_count() -> i64 {
    let mut conn = establish_connection();
    crate::schema::moves::table
        .count()
        .get_result(&mut conn)
        .unwrap()
}

pub fn insert_games(games: &Vec<Game>) {
    let mut conn = establish_connection();

    diesel::insert_into(crate::schema::games::table)
        .values(games)
        .execute(&mut conn)
        .unwrap();
}

pub fn insert_moves(moves: &Vec<Move>) {
    let mut conn = establish_connection();

    diesel::insert_into(crate::schema::moves::table)
        .values(moves)
        .execute(&mut conn)
        .unwrap();
}

fn insert_position_with_return_id(position: &Position) -> i32 {
    let mut conn = establish_connection();
    let id = diesel::insert_into(crate::schema::positions::table)
        .values(position)
        .returning(crate::schema::positions::id)
        .execute(&mut conn)
        .unwrap();

    id as i32
}

pub fn create_position(fen: &str) -> i32 {
    let position = Position {
        fen: fen.to_string(),
        ..Default::default()
    };
    insert_position_with_return_id(&position)
}

pub fn get_position_id_by_fen(fen: &str) -> Option<i32> {
    let mut conn = establish_connection();
    crate::schema::positions::table
        .filter(crate::schema::positions::fen.eq(fen))
        .select(crate::schema::positions::id)
        .first(&mut conn)
        .ok()
}
