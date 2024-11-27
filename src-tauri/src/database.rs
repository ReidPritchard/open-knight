use diesel::{alias, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use crate::{
    convert::moves_to_api_moves,
    models::{APIGame, APIMove, Game, Header, Move, Position},
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

    #[cfg(not(test))]
    pub fn establish_connection() -> Result<SqliteConnection, DatabaseError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| DatabaseError::ConfigError("DATABASE_URL must be set".to_string()))?;
        let mut conn = SqliteConnection::establish(&database_url)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // Run migrations in production as well to ensure schema is up to date
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| DatabaseError::ConnectionError(format!("Migration failed: {}", e)))?;

        Ok(conn)
    }

    #[cfg(test)]
    pub fn establish_connection() -> Result<SqliteConnection, DatabaseError> {
        // For tests, always use an in-memory database
        let mut conn = SqliteConnection::establish(":memory:")
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // Run migrations for test database
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| DatabaseError::ConnectionError(format!("Migration failed: {}", e)))?;

        Ok(conn)
    }

    pub fn empty_db() -> Result<(), DatabaseError> {
        let mut conn = establish_connection()?;
        diesel::delete(crate::schema::games::table).execute(&mut conn)?;
        diesel::delete(crate::schema::moves::table).execute(&mut conn)?;
        diesel::delete(crate::schema::headers::table).execute(&mut conn)?;
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

    /// Get all games with their headers in a single query
    pub fn get_all_games_with_headers() -> Result<Vec<(Game, Vec<Header>)>, DatabaseError> {
        use diesel::prelude::*;
        let mut conn = setup::establish_connection()?;

        // Get all games and headers in a single query using a left join
        // We use left join to ensure we get games even if they have no headers
        let results: Vec<(Game, Option<Header>)> = crate::schema::games::table
            .left_join(
                crate::schema::headers::table
                    .on(crate::schema::games::id.eq(crate::schema::headers::game_id)),
            )
            .select((
                crate::schema::games::all_columns,
                crate::schema::headers::all_columns.nullable(),
            ))
            .load(&mut conn)?;

        // Group the results by game
        let mut games_map: std::collections::HashMap<i32, (Game, Vec<Header>)> = HashMap::new();

        for (game, header_opt) in results {
            let game_id = game.id.unwrap();
            let entry = games_map
                .entry(game_id)
                .or_insert_with(|| (game, Vec::new()));

            if let Some(header) = header_opt {
                // entry: (game, vec<header>)
                entry.1.push(header);
            }
        }

        Ok(games_map.into_values().collect())
    }

    pub fn get_full_game_by_id(id: i32) -> Result<Option<APIGame>, DatabaseError> {
        let mut conn = setup::establish_connection()?;

        // Get the game
        let game = match crate::schema::games::table.find(id).first(&mut conn) {
            Ok(g) => g,
            Err(diesel::result::Error::NotFound) => return Ok(None),
            Err(e) => return Err(e.into()),
        };

        // Get the moves with positions
        let moves = move_::get_moves_by_game_id(id)?;

        // Get the headers
        let headers = header::get_headers_by_game_id(id)?;

        Ok(Some(APIGame::from((game, moves, headers))))
    }

    pub fn insert_games(games: &[Game]) -> Result<(), DatabaseError> {
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

    pub fn insert_moves(moves: &[Move]) -> Result<(), DatabaseError> {
        let mut conn = setup::establish_connection()?;
        diesel::insert_into(crate::schema::moves::table)
            .values(moves)
            .execute(&mut conn)?;
        Ok(())
    }
}

// === Header Queries ===

pub mod header {
    use super::*;

    pub fn get_headers_by_game_id(id: i32) -> Result<Vec<Header>, DatabaseError> {
        let mut conn = setup::establish_connection()?;
        Ok(crate::schema::headers::table
            .filter(crate::schema::headers::game_id.eq(id))
            .load(&mut conn)?)
    }

    pub fn insert_headers(headers: &[Header]) -> Result<(), DatabaseError> {
        let mut conn = setup::establish_connection()?;
        diesel::insert_into(crate::schema::headers::table)
            .values(headers)
            .execute(&mut conn)?;
        Ok(())
    }
}

// === Position Queries ===

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
