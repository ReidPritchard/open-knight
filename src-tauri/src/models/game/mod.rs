pub mod conversion;
pub mod database;
pub mod metadata;
pub mod player_ops;
pub mod structs;

use ok_parse::pgn::parse_pgn_games;
use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::parse::load_moves_from_db;
use crate::entities::*;
use crate::models::{ChessMoveTree, ChessPosition};
use crate::utils::AppError;

// Re-export structs for public API
pub use structs::{ChessGame, ChessOpening, ChessPlayer, ChessTournament};

impl ChessGame {
    /// Creates a new chess game with default starting position
    pub async fn new(variant: &str, db: &DatabaseConnection) -> Result<Self, AppError> {
        if variant != "standard" {
            // Display warning message
            eprintln!(
                "Warning: Non-standard chess variant ({}) support is in progress and may not work as expected.",
                variant
            );
        }

        // Create default players
        let white_player_id = player_ops::create_default_player(db, "White Player").await?;
        let black_player_id = player_ops::create_default_player(db, "Black Player").await?;

        let starting_position = ChessPosition::default();
        let current_date = chrono::Utc::now().to_rfc3339();

        // Create a new game struct
        let mut game = ChessGame {
            id: 0, // Will be set after database insertion
            white_player: ChessPlayer {
                id: white_player_id,
                name: "White Player".to_string(),
                elo: None,
                country: None,
            },
            black_player: ChessPlayer {
                id: black_player_id,
                name: "Black Player".to_string(),
                elo: None,
                country: None,
            },
            tournament: None,
            opening: None,
            result: "*".to_string(),
            round: None,
            date: current_date.clone(),
            headers: vec![],
            move_tree: ChessMoveTree::default(),
            tags: vec!["local".to_string()],
            fen: Some(starting_position.fen),
            pgn: None,
            variant: variant.to_string(),
        };

        // Insert into database
        let db_game_model = game::ActiveModel {
            white_player_id: Set(white_player_id),
            black_player_id: Set(black_player_id),
            tournament_id: Set(None),
            opening_id: Set(None),
            result: Set(Some(game.result.clone())),
            round_number: Set(game.round),
            date_played: Set(Some(current_date)),
            fen: Set(game.fen.clone()),
            pgn: Set("".to_string()),
            created_at: Set(Some(chrono::Utc::now())),
            ..Default::default()
        };

        let insert_result = game::Entity::insert(db_game_model)
            .exec(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to create new game: {}", e)))?;

        game.id = insert_result.last_insert_id;
        Ok(game)
    }

    /// Default constructor for a new chess game without database interaction
    pub fn new_default() -> Self {
        Self {
            id: 0,
            white_player: player_ops::new_player("White Player"),
            black_player: player_ops::new_player("Black Player"),
            tournament: None,
            opening: None,
            result: "*".to_string(),
            round: None,
            date: chrono::Utc::now().to_rfc3339(),
            headers: vec![],
            move_tree: ChessMoveTree::default(),
            tags: vec![],
            fen: Some(ChessPosition::default().fen),
            pgn: None,
            variant: "standard".to_string(),
        }
    }

    /// Loads a chess game from the database by ID
    pub async fn load(db: &DatabaseConnection, game_id: i32) -> Result<Self, AppError> {
        // Load the game
        let game = game::Entity::find_by_id(game_id)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to query game: {}", e)))?
            .ok_or_else(|| {
                AppError::DatabaseError(format!("Game with ID {} not found", game_id))
            })?;

        // Load headers
        let headers = game_header::Entity::find()
            .filter(game_header::Column::GameId.eq(game_id))
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to load headers: {}", e)))?
            .into_iter()
            .map(|h| (h.header_name, h.header_value))
            .collect();

        // Load players
        let white_player = player::Entity::find_by_id(game.white_player_id)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to query white player: {}", e)))?
            .ok_or_else(|| {
                AppError::DatabaseError(format!(
                    "White player with ID {} not found",
                    game.white_player_id
                ))
            })?;

        let black_player = player::Entity::find_by_id(game.black_player_id)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to query black player: {}", e)))?
            .ok_or_else(|| {
                AppError::DatabaseError(format!(
                    "Black player with ID {} not found",
                    game.black_player_id
                ))
            })?;

        // Load tournament if exists
        let tournament = if let Some(tournament_id) = game.tournament_id {
            Some(metadata::load_tournament(db, tournament_id).await?)
        } else {
            None
        };

        // Load opening if exists
        let opening = if let Some(opening_id) = game.opening_id {
            Some(metadata::load_opening(db, opening_id).await?)
        } else {
            None
        };

        // Create the game object
        Ok(ChessGame {
            id: game.game_id,
            white_player: ChessPlayer {
                id: white_player.player_id,
                name: white_player.name,
                elo: white_player.elo_rating,
                country: white_player.country_code,
            },
            black_player: ChessPlayer {
                id: black_player.player_id,
                name: black_player.name,
                elo: black_player.elo_rating,
                country: black_player.country_code,
            },
            tournament,
            opening,
            result: game.result.unwrap_or_else(|| "*".to_string()),
            round: game.round_number,
            date: game.date_played.unwrap_or_else(|| "????-??-??".to_string()),
            headers,
            move_tree: ChessMoveTree::default(),
            tags: Vec::new(),
            fen: game.fen,
            pgn: Some(game.pgn),
            variant: game.variant.unwrap_or_else(|| "standard".to_string()),
        })
    }

    /// Loads the move tree for this game from the database
    pub async fn load_moves(&mut self, db: &DatabaseConnection) -> Result<(), AppError> {
        let starting_position = ChessPosition::from_fen(self.fen.clone(), None)
            .map_err(|e| AppError::ChessError(format!("Invalid FEN in game: {}", e)))?;

        let move_tree = load_moves_from_db(db, self.id, starting_position)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to load moves: {}", e)))?;

        self.move_tree = move_tree;
        Ok(())
    }

    /// Loads the tags for this game from the database
    pub async fn load_tags(&mut self, db: &DatabaseConnection) -> Result<(), AppError> {
        let tags = game_tag::Entity::find()
            .filter(game_tag::Column::GameId.eq(self.id))
            .find_also_related(tag::Entity)
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to load tags: {}", e)))?;

        self.tags = tags
            .into_iter()
            .filter_map(|(_, tag)| tag)
            .map(|t| format!("[{} \"{}\"]", t.name, t.description.unwrap_or_default()))
            .collect();

        Ok(())
    }

    /// Saves multiple chess games from PGN format to the database
    pub async fn save_from_pgn(db: &DatabaseConnection, pgn: &str) -> Result<Vec<Self>, AppError> {
        let chess_games: Vec<Self> = parse_pgn_games(pgn)
            .map(|games| games.into_iter().map(|g| g.into()).collect())
            .map_err(|e| AppError::ParseError(e.into_iter().next().unwrap()))?;

        if chess_games.is_empty() {
            return Err(AppError::GeneralError("No games found in PGN".to_string()));
        }

        let mut saved_games = Vec::new();

        // Process games in smaller batches to avoid overwhelming the database
        const BATCH_SIZE: usize = 10;
        let num_batches = chess_games.len().div_ceil(BATCH_SIZE);
        for (batch_index, batch) in chess_games.chunks(BATCH_SIZE).enumerate() {
            println!("Processing batch {} of {}", batch_index + 1, num_batches);

            let mut batch_results = Vec::new();

            for chess_game in batch {
                match Self::save_single_game(db, chess_game).await {
                    Ok(game) => batch_results.push(game),
                    Err(e) => {
                        eprintln!("Error saving game: {}", e);
                        // Continue with other games instead of failing the entire batch
                    }
                }
            }

            saved_games.extend(batch_results);
        }

        println!(
            "Successfully saved {} out of {} games",
            saved_games.len(),
            chess_games.len()
        );
        Ok(saved_games)
    }

    /// Saves a single chess game to the database
    async fn save_single_game(
        db: &DatabaseConnection,
        chess_game: &ChessGame,
    ) -> Result<Self, AppError> {
        let txn = database::begin_transaction(db).await?;

        let result = async {
            let mut game = chess_game.clone();

            // Save metadata (players, tournament, opening)
            let metadata = database::save_game_metadata(&txn, &game).await?;
            game.white_player.id = metadata.white_player_id;
            game.black_player.id = metadata.black_player_id;

            // Save game
            let game_model = database::create_game_model(&game, &metadata, None);
            let result = game::Entity::insert(game_model)
                .exec(&txn)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to save game: {}", e)))?;
            game.id = result.last_insert_id;

            // Save moves
            database::save_game_moves(&txn, &game, false).await?;

            Ok::<_, AppError>(game)
        }
        .await;

        match result {
            Ok(game) => {
                database::commit_transaction(txn).await?;
                Ok(game)
            }
            Err(e) => {
                database::rollback_transaction(txn).await;
                Err(e)
            }
        }
    }

    /// Makes a move in the game from the current position
    pub async fn make_uci_move(&mut self, uci_move_notation: &str) -> Result<(), AppError> {
        if uci_move_notation.trim().is_empty() {
            return Err(AppError::ChessError(
                "Move notation cannot be empty".to_string(),
            ));
        }

        self.move_tree.make_uci_move(uci_move_notation);
        Ok(())
    }

    /// Deletes a game from the database
    pub async fn delete(db: &DatabaseConnection, game_id: i32) -> Result<(), AppError> {
        game::Entity::delete_by_id(game_id)
            .exec(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete game: {}", e)))?;

        Ok(())
    }

    /// Saves the game to the database
    pub async fn save(self, db: &DatabaseConnection) -> Result<Self, AppError> {
        Self::save_single_game(db, &self).await
    }

    /// Updates the game in the database
    pub async fn update(self, db: &DatabaseConnection) -> Result<Self, AppError> {
        // Validate that the game has a valid ID
        if self.id <= 0 {
            return Err(AppError::ChessError(
                "Cannot update game: invalid game ID".to_string(),
            ));
        }

        let txn = database::begin_transaction(db).await?;

        let result = async {
            let mut game = self.clone();

            // Update metadata (players, tournament, opening)
            let metadata = database::save_game_metadata(&txn, &game).await?;
            game.white_player.id = metadata.white_player_id;
            game.black_player.id = metadata.black_player_id;

            // Update the game record
            let game_model = database::create_game_model(&game, &metadata, Some(game.id));
            game::Entity::update(game_model)
                .exec(&txn)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update game: {}", e)))?;

            // Save moves (delete existing ones first)
            database::save_game_moves(&txn, &game, true).await?;

            Ok::<_, AppError>(game)
        }
        .await;

        match result {
            Ok(game) => {
                database::commit_transaction(txn).await?;
                Ok(game)
            }
            Err(e) => {
                database::rollback_transaction(txn).await;
                Err(e)
            }
        }
    }
}
