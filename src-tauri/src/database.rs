use diesel::{
    alias,
    prelude::*,
    r2d2::{self as r2d2_diesel, ConnectionManager},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use crate::{
    convert::moves_to_api_moves,
    models::{APIGame, APIMove, Game, Header, Move, Position},
};

// Type aliases for cleaner code
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

// Global pool stored in AppState instead of a static
pub struct Database {
    pool: DbPool,
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(diesel::result::Error),
    ConfigError(String),
    PoolError(r2d2::Error),
    DieselPoolError(r2d2_diesel::Error),
    TransactionError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            DatabaseError::QueryError(e) => write!(f, "Query error: {}", e),
            DatabaseError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            DatabaseError::PoolError(e) => write!(f, "Pool error: {}", e),
            DatabaseError::DieselPoolError(e) => write!(f, "Diesel pool error: {}", e),
            DatabaseError::TransactionError(e) => write!(f, "Transaction error: {}", e),
        }
    }
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(err: diesel::result::Error) -> Self {
        DatabaseError::QueryError(err)
    }
}

impl From<r2d2_diesel::Error> for DatabaseError {
    fn from(err: r2d2_diesel::Error) -> Self {
        DatabaseError::DieselPoolError(err)
    }
}

impl From<r2d2::Error> for DatabaseError {
    fn from(err: r2d2::Error) -> Self {
        DatabaseError::PoolError(err)
    }
}

impl Database {
    pub fn new() -> Result<Self, DatabaseError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| DatabaseError::ConfigError("DATABASE_URL must be set".to_string()))?;

        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder().max_size(5).build(manager)?;

        // Run migrations on one connection
        let mut conn = pool.get().map_err(|e| DatabaseError::PoolError(e))?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| DatabaseError::ConnectionError(format!("Migration failed: {}", e)))?;

        Ok(Database { pool })
    }

    pub fn get_pool(&self) -> &DbPool {
        &self.pool
    }

    pub fn with_connection<F, T>(&self, f: F) -> Result<T, DatabaseError>
    where
        F: FnOnce(&mut DbConnection) -> Result<T, DatabaseError>,
    {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::PoolError(e))?;
        f(&mut conn)
    }

    pub fn transaction<F, T>(&self, f: F) -> Result<T, DatabaseError>
    where
        F: FnOnce(&mut DbConnection) -> Result<T, DatabaseError>,
    {
        let mut conn = self.pool.get().map_err(|e| DatabaseError::PoolError(e))?;
        conn.transaction(|conn| f(conn))
    }
}

// === Setup Functions ===

pub mod setup {
    use super::*;

    pub fn empty_db(db: &Database) -> Result<(), DatabaseError> {
        db.with_connection(|conn| {
            diesel::delete(crate::schema::games::table).execute(conn)?;
            diesel::delete(crate::schema::moves::table).execute(conn)?;
            diesel::delete(crate::schema::headers::table).execute(conn)?;
            diesel::delete(crate::schema::positions::table).execute(conn)?;
            Ok(())
        })
    }
}

// === Game Queries ===

pub mod game {
    use super::*;

    pub fn get_game_id_count(db: &Database) -> Result<i64, DatabaseError> {
        db.with_connection(|conn| Ok(crate::schema::games::table.count().get_result(conn)?))
    }

    pub fn get_all_games(db: &Database) -> Result<Vec<Game>, DatabaseError> {
        db.with_connection(|conn| Ok(crate::schema::games::table.load(conn)?))
    }

    pub fn get_all_games_with_headers(
        db: &Database,
    ) -> Result<Vec<(Game, Vec<Header>)>, DatabaseError> {
        db.with_connection(|conn| {
            // Get all games and headers in a single query using a left join
            let results: Vec<(Game, Option<Header>)> = crate::schema::games::table
                .left_join(
                    crate::schema::headers::table
                        .on(crate::schema::games::id.eq(crate::schema::headers::game_id)),
                )
                .select((
                    crate::schema::games::all_columns,
                    crate::schema::headers::all_columns.nullable(),
                ))
                .load(conn)?;

            // Group the results by game
            let mut games_map: HashMap<i32, (Game, Vec<Header>)> = HashMap::new();

            for (game, header_opt) in results {
                let game_id = game.id.unwrap();
                let entry = games_map
                    .entry(game_id)
                    .or_insert_with(|| (game, Vec::new()));

                if let Some(header) = header_opt {
                    entry.1.push(header);
                }
            }

            Ok(games_map.into_values().collect())
        })
    }

    pub fn get_full_game_by_id(db: &Database, id: i32) -> Result<Option<APIGame>, DatabaseError> {
        db.with_connection(|conn| {
            // Get the game
            let game = match crate::schema::games::table.find(id).first(conn) {
                Ok(g) => g,
                Err(diesel::result::Error::NotFound) => return Ok(None),
                Err(e) => return Err(e.into()),
            };

            // Get the moves with positions
            let moves = move_::get_moves_by_game_id(db, id)?;

            // Get the headers
            let headers = header::get_headers_by_game_id(db, id)?;

            Ok(Some(APIGame::from((game, moves, headers))))
        })
    }

    pub fn insert_games_returning_ids_with_conn(
        conn: &mut DbConnection,
        games: &[Game],
    ) -> Result<Vec<i32>, DatabaseError> {
        let mut ids = Vec::new();
        for game in games {
            let id = diesel::insert_into(crate::schema::games::table)
                .values(game)
                .returning(crate::schema::games::id)
                .get_result(conn)?;
            ids.push(id);
        }
        Ok(ids)
    }

    pub fn insert_games_returning_ids(
        db: &Database,
        games: &[Game],
    ) -> Result<Vec<i32>, DatabaseError> {
        db.with_connection(|conn| insert_games_returning_ids_with_conn(conn, games))
    }
}

// === Move Queries ===

pub mod move_ {
    use super::*;

    pub fn insert_moves_with_conn(
        conn: &mut DbConnection,
        moves: &[Move],
    ) -> Result<(), DatabaseError> {
        diesel::insert_into(crate::schema::moves::table)
            .values(moves)
            .execute(conn)?;
        Ok(())
    }

    pub fn insert_moves(db: &Database, moves: &[Move]) -> Result<(), DatabaseError> {
        db.with_connection(|conn| insert_moves_with_conn(conn, moves))
    }

    pub fn get_moves_by_game_id(db: &Database, id: i32) -> Result<Vec<APIMove>, DatabaseError> {
        db.with_connection(|conn| {
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
                .load::<(Move, Position, Position)>(conn)?;

            Ok(moves_to_api_moves(moves))
        })
    }
}

// === Header Queries ===

pub mod header {
    use super::*;

    pub fn insert_headers_with_conn(
        conn: &mut DbConnection,
        headers: &[Header],
    ) -> Result<(), DatabaseError> {
        diesel::insert_into(crate::schema::headers::table)
            .values(headers)
            .execute(conn)?;
        Ok(())
    }

    pub fn insert_headers(db: &Database, headers: &[Header]) -> Result<(), DatabaseError> {
        db.with_connection(|conn| insert_headers_with_conn(conn, headers))
    }

    pub fn get_headers_by_game_id(db: &Database, id: i32) -> Result<Vec<Header>, DatabaseError> {
        db.with_connection(|conn| {
            Ok(crate::schema::headers::table
                .filter(crate::schema::headers::game_id.eq(id))
                .load(conn)?)
        })
    }
}

// === Position Queries ===

pub mod position {
    use super::*;

    pub fn get_position_id_by_fen_with_conn(
        conn: &mut DbConnection,
        fen: &str,
    ) -> Result<Option<i32>, DatabaseError> {
        use crate::schema::positions::dsl::*;

        let result = positions
            .filter(fen.eq(fen))
            .select(id)
            .first(conn)
            .optional()?;

        Ok(result)
    }

    pub fn create_position_with_conn(
        conn: &mut DbConnection,
        fen_str: &str,
    ) -> Result<i32, DatabaseError> {
        use crate::schema::positions::dsl::*;

        diesel::insert_into(positions)
            .values(fen.eq(fen_str))
            .returning(id)
            .get_result(conn)
            .map_err(|e| e.into())
    }

    pub fn get_position_id_by_fen(db: &Database, fen: &str) -> Result<Option<i32>, DatabaseError> {
        db.with_connection(|conn| get_position_id_by_fen_with_conn(conn, fen))
    }

    pub fn create_position(db: &Database, fen: &str) -> Result<i32, DatabaseError> {
        db.with_connection(|conn| create_position_with_conn(conn, fen))
    }
}
