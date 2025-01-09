use std::collections::HashMap;
use tauri::State;
use tauri_plugin_sql::{Migration, MigrationKind};

use crate::{
    convert::moves_to_api_moves,
    models::{APIGame, APIMove, Game, Header, Move, Position},
};

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    ConfigError(String),
    TransactionError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            DatabaseError::QueryError(e) => write!(f, "Query error: {}", e),
            DatabaseError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            DatabaseError::TransactionError(e) => write!(f, "Transaction error: {}", e),
        }
    }
}

pub struct Database {
    pool: tauri_plugin_sql::SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, DatabaseError> {
        let pool = tauri_plugin_sql::SqlitePool::connect("sqlite:chess.db")
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Database { pool })
    }

    pub fn get_pool(&self) -> &tauri_plugin_sql::SqlitePool {
        &self.pool
    }
}

// === Setup Functions ===
pub mod setup {
    use super::*;

    pub async fn empty_db(db: &Database) -> Result<(), DatabaseError> {
        let pool = db.get_pool();
        pool.execute("DELETE FROM games", ())
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        pool.execute("DELETE FROM moves", ())
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        pool.execute("DELETE FROM headers", ())
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        pool.execute("DELETE FROM positions", ())
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(())
    }
}

// === Game Queries ===
pub mod game {
    use super::*;

    pub async fn get_game_id_count(db: &Database) -> Result<i64, DatabaseError> {
        let pool = db.get_pool();
        let count: i64 = pool
            .fetch_one("SELECT COUNT(*) as count FROM games", ())
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .get("count");
        Ok(count)
    }

    pub async fn get_all_games(db: &Database) -> Result<Vec<Game>, DatabaseError> {
        let pool = db.get_pool();
        let games = pool
            .fetch_all("SELECT * FROM games", ())
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(games
            .into_iter()
            .map(|row| Game {
                id: Some(row.get("id")),
                pgn: row.get("pgn"),
                player_white: row.get("player_white"),
                player_black: row.get("player_black"),
                event: row.get("event"),
                date_text: row.get("date_text"),
                result: row.get("result"),
                annotations: row.get("annotations"),
                opening_name: row.get("opening_name"),
            })
            .collect())
    }

    pub async fn get_all_games_with_headers(
        db: &Database,
    ) -> Result<Vec<(Game, Vec<Header>)>, DatabaseError> {
        let pool = db.get_pool();
        let rows = pool
            .fetch_all(
                "SELECT g.*, h.id as header_id, h.header_key, h.header_value 
                FROM games g 
                LEFT JOIN headers h ON g.id = h.game_id",
                (),
            )
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let mut games_map: HashMap<i32, (Game, Vec<Header>)> = HashMap::new();

        for row in rows {
            let game_id: i32 = row.get("id");
            let game = Game {
                id: Some(game_id),
                pgn: row.get("pgn"),
                player_white: row.get("player_white"),
                player_black: row.get("player_black"),
                event: row.get("event"),
                date_text: row.get("date_text"),
                result: row.get("result"),
                annotations: row.get("annotations"),
                opening_name: row.get("opening_name"),
            };

            let header_id: Option<i32> = row.get("header_id");
            if let Some(_) = header_id {
                let header = Header {
                    id: header_id,
                    game_id,
                    header_key: row.get("header_key"),
                    header_value: row.get("header_value"),
                };

                games_map
                    .entry(game_id)
                    .or_insert_with(|| (game, Vec::new()))
                    .1
                    .push(header);
            } else {
                games_map
                    .entry(game_id)
                    .or_insert_with(|| (game, Vec::new()));
            }
        }

        Ok(games_map.into_values().collect())
    }

    pub async fn get_full_game_by_id(
        db: &Database,
        id: i32,
    ) -> Result<Option<APIGame>, DatabaseError> {
        let pool = db.get_pool();

        // Get the game
        let game = match pool
            .fetch_optional("SELECT * FROM games WHERE id = ?", (id,))
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
        {
            Some(row) => Game {
                id: Some(row.get("id")),
                pgn: row.get("pgn"),
                player_white: row.get("player_white"),
                player_black: row.get("player_black"),
                event: row.get("event"),
                date_text: row.get("date_text"),
                result: row.get("result"),
                annotations: row.get("annotations"),
                opening_name: row.get("opening_name"),
            },
            None => return Ok(None),
        };

        // Get the moves
        let moves = move_::get_moves_by_game_id(db, id).await?;

        // Get the headers
        let headers = header::get_headers_by_game_id(db, id).await?;

        Ok(Some(APIGame::from((game, moves, headers))))
    }

    pub async fn insert_games_returning_ids(
        db: &Database,
        games: &[Game],
    ) -> Result<Vec<i32>, DatabaseError> {
        let pool = db.get_pool();
        let mut ids = Vec::new();

        for game in games {
            let id: i32 = pool
                .execute(
                    "INSERT INTO games (pgn, player_white, player_black, event, date_text, result, annotations, opening_name) 
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING id",
                    (
                        &game.pgn,
                        &game.player_white,
                        &game.player_black,
                        &game.event,
                        &game.date_text,
                        &game.result,
                        &game.annotations,
                        &game.opening_name,
                    ),
                ).await
                .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
            ids.push(id);
        }

        Ok(ids)
    }
}

// === Move Queries ===
pub mod move_ {
    use super::*;

    pub async fn insert_moves(db: &Database, moves: &[Move]) -> Result<(), DatabaseError> {
        let pool = db.get_pool();

        for m in moves {
            pool.execute(
                "INSERT INTO moves (game_id, move_number, move_san, annotation, variation_order, parent_position_id, child_position_id) 
                VALUES (?, ?, ?, ?, ?, ?, ?)",
                (
                    m.game_id,
                    m.move_number,
                    &m.move_san,
                    &m.annotation,
                    m.variation_order,
                    m.parent_position_id,
                    m.child_position_id,
                ),
            ).await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn get_moves_by_game_id(
        db: &Database,
        id: i32,
    ) -> Result<Vec<APIMove>, DatabaseError> {
        let pool = db.get_pool();

        let rows = pool
            .fetch_all(
                "SELECT m.*, pp.fen as parent_fen, cp.fen as child_fen 
                FROM moves m
                INNER JOIN positions pp ON m.parent_position_id = pp.id
                INNER JOIN positions cp ON m.child_position_id = cp.id
                WHERE m.game_id = ?",
                (id,),
            )
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let moves = rows
            .into_iter()
            .map(|row| Move {
                id: Some(row.get("id")),
                game_id: row.get("game_id"),
                move_number: row.get("move_number"),
                move_san: row.get("move_san"),
                annotation: row.get("annotation"),
                variation_order: row.get("variation_order"),
                parent_position_id: row.get("parent_position_id"),
                child_position_id: row.get("child_position_id"),
            })
            .collect();

        Ok(moves_to_api_moves(moves))
    }
}

// === Header Queries ===
pub mod header {
    use super::*;

    pub async fn insert_headers(db: &Database, headers: &[Header]) -> Result<(), DatabaseError> {
        let pool = db.get_pool();

        for header in headers {
            pool.execute(
                "INSERT INTO headers (game_id, header_key, header_value) VALUES (?, ?, ?)",
                (header.game_id, &header.header_key, &header.header_value),
            )
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn get_headers_by_game_id(
        db: &Database,
        id: i32,
    ) -> Result<Vec<Header>, DatabaseError> {
        let pool = db.get_pool();

        let rows = pool
            .fetch_all("SELECT * FROM headers WHERE game_id = ?", (id,))
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Header {
                id: Some(row.get("id")),
                game_id: row.get("game_id"),
                header_key: row.get("header_key"),
                header_value: row.get("header_value"),
            })
            .collect())
    }
}

// === Position Queries ===
pub mod position {
    use super::*;

    pub async fn get_position_id_by_fen(
        db: &Database,
        fen: &str,
    ) -> Result<Option<i32>, DatabaseError> {
        let pool = db.get_pool();

        let row = pool
            .fetch_optional("SELECT id FROM positions WHERE fen = ?", (fen,))
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(row.map(|r| r.get("id")))
    }

    pub async fn create_position(db: &Database, fen: &str) -> Result<i32, DatabaseError> {
        let pool = db.get_pool();

        let id: i32 = pool
            .execute(
                "INSERT INTO positions (fen) VALUES (?) RETURNING id",
                (fen,),
            )
            .await
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(id)
    }
}
