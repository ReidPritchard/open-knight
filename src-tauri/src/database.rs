use diesel::{alias, prelude::*};
use dotenvy::dotenv;
use std::env;

use crate::{
    api_types::APIMove,
    convert::moves_to_api_moves,
    models::{Game, Move, Position},
};

// === Connection and Setup ===

pub mod setup {
    use super::*;

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
        diesel::delete(crate::schema::positions::table)
            .execute(&mut conn)
            .unwrap();
    }
}

// === Game Queries ===

pub mod game {

    use crate::api_types::APIGame;

    use super::*;

    pub fn get_game_id_count() -> i64 {
        let mut conn = setup::establish_connection();
        crate::schema::games::table
            .count()
            .get_result(&mut conn)
            .unwrap()
    }

    pub fn get_all_games() -> Vec<Game> {
        let mut conn = setup::establish_connection();
        crate::schema::games::table.load(&mut conn).unwrap()
    }

    pub fn get_full_game_by_id(id: i32) -> Option<APIGame> {
        let mut conn = setup::establish_connection();

        // Get the basic game info
        let game = crate::schema::games::table
            .find(id)
            .first(&mut conn)
            .unwrap();

        // Get the moves
        let moves = move_::get_moves_by_game_id(id);

        // Combine them
        Some(APIGame::from((game, moves)))
    }

    /**
     * Get all games joined with their moves (including move positions)
     *
     * Currently we use N+1 queries to get all games and their moves,
     * if it becomes a problem we can optimize it by using a join.
     */
    pub fn get_all_full_games() -> Vec<APIGame> {
        let mut conn = setup::establish_connection();

        // Get all games first
        let games: Vec<Game> = crate::schema::games::table.load(&mut conn).unwrap();

        // Create the result vector
        games
            .into_iter()
            .map(|game| {
                // For each game, get its moves
                let moves = move_::get_moves_by_game_id(game.id.unwrap());
                APIGame::from((game, moves))
            })
            .collect()
    }

    pub fn insert_games(games: &Vec<Game>) {
        let mut conn = setup::establish_connection();

        diesel::insert_into(crate::schema::games::table)
            .values(games)
            .execute(&mut conn)
            .unwrap();
    }
}

// === Move Queries ===

pub mod move_ {
    use super::*;

    pub fn get_all_moves() -> Vec<APIMove> {
        let mut conn = setup::establish_connection();

        let (parent_position, child_position) = alias!(
            crate::schema::positions as parent_position,
            crate::schema::positions as child_position
        );

        let moves = crate::schema::moves::table
            .inner_join(
                parent_position.on(crate::schema::moves::parent_position_id
                    .eq(parent_position.field(crate::schema::positions::id))),
            )
            .inner_join(
                child_position.on(crate::schema::moves::child_position_id
                    .eq(child_position.field(crate::schema::positions::id))),
            )
            .select((
                crate::schema::moves::all_columns,
                parent_position.fields((
                    crate::schema::positions::id,
                    crate::schema::positions::fen,
                    crate::schema::positions::annotation,
                )),
                child_position.fields((
                    crate::schema::positions::id,
                    crate::schema::positions::fen,
                    crate::schema::positions::annotation,
                )),
            ))
            .load::<(Move, Position, Position)>(&mut conn)
            .unwrap();

        // Convert to APIMoves
        moves_to_api_moves(moves)
    }

    pub fn get_moves_by_game_id(id: i32) -> Vec<APIMove> {
        let mut conn = setup::establish_connection();

        let (parent_position, child_position) = alias!(
            crate::schema::positions as parent_position,
            crate::schema::positions as child_position
        );

        let moves = crate::schema::moves::table
            .filter(crate::schema::moves::game_id.eq(id))
            .inner_join(
                parent_position.on(crate::schema::moves::parent_position_id
                    .eq(parent_position.field(crate::schema::positions::id))),
            )
            .inner_join(
                child_position.on(crate::schema::moves::child_position_id
                    .eq(child_position.field(crate::schema::positions::id))),
            )
            .select((
                crate::schema::moves::all_columns,
                parent_position.fields((
                    crate::schema::positions::id,
                    crate::schema::positions::fen,
                    crate::schema::positions::annotation,
                )),
                child_position.fields((
                    crate::schema::positions::id,
                    crate::schema::positions::fen,
                    crate::schema::positions::annotation,
                )),
            ))
            .load::<(Move, Position, Position)>(&mut conn)
            .unwrap();

        moves_to_api_moves(moves)
    }

    pub fn insert_moves(moves: &Vec<Move>) {
        let mut conn = setup::establish_connection();

        diesel::insert_into(crate::schema::moves::table)
            .values(moves)
            .execute(&mut conn)
            .unwrap();
    }
}

// === Position Queries and Insertions ===

pub mod position {
    use super::*;

    fn insert_position_with_return_id(position: &Position) -> i32 {
        let mut conn = setup::establish_connection();
        let id = diesel::insert_into(crate::schema::positions::table)
            .values(position)
            .returning(crate::schema::positions::id)
            .execute(&mut conn)
            .unwrap();

        id as i32
    }

    pub fn create_position(fen: &str) -> i32 {
        println!("Creating position: {}", fen);
        let position = Position {
            fen: fen.to_string(),
            ..Default::default()
        };
        insert_position_with_return_id(&position)
    }

    pub fn get_position_id_by_fen(fen: &str) -> Option<i32> {
        let mut conn = setup::establish_connection();
        crate::schema::positions::table
            .filter(crate::schema::positions::fen.eq(fen))
            .select(crate::schema::positions::id)
            .first(&mut conn)
            .ok()
    }
}
