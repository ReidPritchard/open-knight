use diesel::{alias, prelude::*};
use dotenvy::dotenv;
use std::env;

use crate::{
    convert::moves_to_api_moves,
    models::{api::*, db::*},
};

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(diesel::result::Error),
    ConfigError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            DatabaseError::QueryError(e) => write!(f, "Query error: {}", e),
            DatabaseError::ConfigError(e) => write!(f, "Configuration error: {}", e),
        }
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(err: diesel::result::Error) -> Self {
        DatabaseError::QueryError(err)
    }
}

// === Connection and Setup ===

pub mod setup {
    use super::*;

    pub fn establish_connection() -> Result<SqliteConnection, DatabaseError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|e| DatabaseError::ConfigError(format!("DATABASE_URL not set: {}", e)))?;

        SqliteConnection::establish(&database_url).map_err(|e| {
            DatabaseError::ConnectionError(format!("Error connecting to {}: {}", database_url, e))
        })
    }

    pub fn empty_db() -> Result<(), DatabaseError> {
        let mut conn = establish_connection()?;
        diesel::delete(crate::schema::games::table).execute(&mut conn)?;
        diesel::delete(crate::schema::moves::table).execute(&mut conn)?;
        diesel::delete(crate::schema::positions::table).execute(&mut conn)?;
        Ok(())
    }
}

// === Game Queries ===

pub mod game {
    use super::*;

    pub fn get_game_id_count() -> Result<i64, DatabaseError> {
        let mut conn = setup::establish_connection()?;
        Ok(crate::schema::games::table.count().get_result(&mut conn)?)
    }

    pub fn get_all_games() -> Result<Vec<Game>, DatabaseError> {
        let mut conn = setup::establish_connection()?;
        Ok(crate::schema::games::table.load(&mut conn)?)
    }

    pub fn get_full_game_by_id(id: i32) -> Result<Option<APIGame>, DatabaseError> {
        let mut conn = setup::establish_connection()?;

        let game = match crate::schema::games::table.find(id).first(&mut conn) {
            Ok(g) => g,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        let moves = move_::get_moves_by_game_id(id)?;
        Ok(Some(APIGame::from((game, moves))))
    }

    pub fn get_all_full_games() -> Result<Vec<APIGame>, DatabaseError> {
        let games = get_all_games()?;

        Ok(games
            .into_iter()
            .map(|game| {
                let moves = move_::get_moves_by_game_id(game.id.unwrap())?;
                Ok(APIGame::from((game, moves)))
            })
            .collect::<Result<Vec<_>, DatabaseError>>()?)
    }

    pub fn insert_games(games: &Vec<Game>) -> Result<(), DatabaseError> {
        let mut conn = setup::establish_connection()?;
        diesel::insert_into(crate::schema::games::table)
            .values(games)
            .execute(&mut conn)?;
        Ok(())
    }
}

// === Move Queries ===

pub mod move_ {
    use super::*;

    pub fn get_all_moves() -> Result<Vec<APIMove>, DatabaseError> {
        let mut conn = setup::establish_connection()?;

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
            .load::<(Move, Position, Position)>(&mut conn)?;

        Ok(moves_to_api_moves(moves))
    }

    pub fn get_moves_by_game_id(id: i32) -> Result<Vec<APIMove>, DatabaseError> {
        let mut conn = setup::establish_connection()?;

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
            .load::<(Move, Position, Position)>(&mut conn)?;

        Ok(moves_to_api_moves(moves))
    }

    pub fn insert_moves(moves: &Vec<Move>) -> Result<(), DatabaseError> {
        let mut conn = setup::establish_connection()?;
        diesel::insert_into(crate::schema::moves::table)
            .values(moves)
            .execute(&mut conn)?;
        Ok(())
    }
}

// === Position Queries and Insertions ===

pub mod position {
    use super::*;

    fn insert_position_with_return_id(position: &Position) -> Result<i32, DatabaseError> {
        let mut conn = setup::establish_connection()?;
        let id = diesel::insert_into(crate::schema::positions::table)
            .values(position)
            .returning(crate::schema::positions::id)
            .execute(&mut conn)?;
        Ok(id as i32)
    }

    pub fn create_position(fen: &str) -> Result<i32, DatabaseError> {
        let position = Position {
            fen: fen.to_string(),
            ..Default::default()
        };
        insert_position_with_return_id(&position)
    }

    pub fn get_position_id_by_fen(fen: &str) -> Result<Option<i32>, DatabaseError> {
        let mut conn = setup::establish_connection()?;
        let result = crate::schema::positions::table
            .filter(crate::schema::positions::fen.eq(fen))
            .select(crate::schema::positions::id)
            .first(&mut conn);

        match result {
            Ok(id) => Ok(Some(id)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
