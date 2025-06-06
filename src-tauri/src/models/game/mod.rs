pub mod pgn;

use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono::{self};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::entities::*;
use crate::models::{ChessMoveTree, ChessPosition};
use crate::parse::pgn::PgnToken;
use crate::ts_export;
use crate::utils::AppError;

use super::parse::load_moves_from_db;

impl std::fmt::Display for PgnToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgnToken::MoveNumber(n) => write!(f, "{}.", n),
            PgnToken::Move(m) => write!(f, "{} ", m),
            PgnToken::Result(r) => write!(f, "{}", r),
            PgnToken::Tag(k, v) => write!(f, "[{} \"{}\"]", k, v),
            PgnToken::Comment(c) => write!(f, "{{{}}}", c),
            PgnToken::Variation(v) => write!(
                f,
                "({})",
                v.iter().map(|t| t.to_string()).collect::<String>()
            ),
        }
    }
}

ts_export! {
    pub struct ChessGame {
        pub id: i32,
        pub white_player: ChessPlayer,
        pub black_player: ChessPlayer,
        pub tournament: Option<ChessTournament>,
        pub opening: Option<ChessOpening>,
        pub result: String,
        pub round: Option<i32>,
        pub date: String,
        pub move_tree: ChessMoveTree,
        pub tags: Vec<String>,
        pub fen: Option<String>,
        pub pgn: Option<String>,
    }
}

ts_export! {
    pub struct ChessPlayer {
        pub id: i32,
        pub name: String,
        pub elo: Option<i32>,
        pub country: Option<String>,
    }
}

ts_export! {
    pub struct ChessTournament {
        pub id: i32,
        pub name: String,
        pub tournament_type: Option<String>,
        pub time_control: Option<String>,
        pub start_date: Option<String>,
        pub end_date: Option<String>,
        pub location: Option<String>,
    }
}

ts_export! {
    pub struct ChessOpening {
        pub id: i32,
        pub eco: Option<String>,
        pub name: Option<String>,
        pub variation: Option<String>,
    }
}

impl ChessGame {
    /// Creates a new chess game with default starting position
    ///
    /// # Arguments
    /// * `variant` - The chess variant (currently only "standard" is supported)
    /// * `db` - Database connection for saving the game
    ///
    /// # Returns
    /// * `Result<Self, AppError>` - The created game or an error
    pub async fn new(variant: &str, db: &DatabaseConnection) -> Result<Self, AppError> {
        if variant != "standard" {
            return Err(AppError::ChessError(format!(
                "Unsupported chess variant: {}. Only 'standard' is currently supported.",
                variant
            )));
        }

        // Create default players (these will be updated when the game is actually played)
        let white_player_id = Self::create_default_player(db, "White Player").await?;
        let black_player_id = Self::create_default_player(db, "Black Player").await?;

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
            move_tree: ChessMoveTree::default(),
            tags: vec!["local".to_string()],
            fen: Some(starting_position.fen),
            pgn: None,
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

    /// Creates a default player for new games
    async fn create_default_player(db: &DatabaseConnection, name: &str) -> Result<i32, AppError> {
        let player_model = player::ActiveModel {
            name: Set(name.to_string()),
            elo_rating: Set(None),
            country_code: Set(None),
            created_at: Set(Some(chrono::Utc::now())),
            updated_at: Set(Some(chrono::Utc::now())),
            ..Default::default()
        };

        let result = player::Entity::insert(player_model)
            .exec(db)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to create default player: {}", e))
            })?;

        Ok(result.last_insert_id)
    }

    /// Loads a chess game from the database by ID
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `game_id` - The ID of the game to load
    ///
    /// # Returns
    /// * `Result<Self, AppError>` - The loaded game or an error
    pub async fn load(db: &DatabaseConnection, game_id: i32) -> Result<Self, AppError> {
        // Load the game
        let game = game::Entity::find_by_id(game_id)
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to query game: {}", e)))?
            .ok_or_else(|| {
                AppError::DatabaseError(format!("Game with ID {} not found", game_id))
            })?;

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
            tournament::Entity::find_by_id(tournament_id)
                .one(db)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to query tournament: {}", e)))?
                .map(|t| ChessTournament {
                    id: t.tournament_id,
                    name: t.name,
                    tournament_type: t.r#type,
                    time_control: t.time_control,
                    start_date: t.start_date,
                    end_date: t.end_date,
                    location: t.location,
                })
        } else {
            None
        };

        // Load opening if exists
        let opening = if let Some(opening_id) = game.opening_id {
            opening::Entity::find_by_id(opening_id)
                .one(db)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to query opening: {}", e)))?
                .map(|o| ChessOpening {
                    id: o.opening_id,
                    eco: o.eco_code.map(|s| s.to_string()),
                    name: Some(o.name),
                    variation: o.variation.map(|s| s.to_string()),
                })
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
            move_tree: ChessMoveTree::default(),
            tags: Vec::new(),
            fen: game.fen,
            pgn: Some(game.pgn),
        })
    }

    /// Loads the move tree for this game from the database
    ///
    /// # Arguments
    /// * `db` - Database connection
    ///
    /// # Returns
    /// * `Result<(), AppError>` - Success or an error
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
    ///
    /// # Arguments
    /// * `db` - Database connection
    ///
    /// # Returns
    /// * `Result<(), AppError>` - Success or an error
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

    /// Finds an existing player or creates a new one
    ///
    /// # Arguments
    /// * `db` - Database connection or transaction
    /// * `player` - Player information to save or find
    ///
    /// # Returns
    /// * `Result<i32, AppError>` - The player ID or an error
    async fn save_or_find_player<C>(db: &C, player: &ChessPlayer) -> Result<i32, AppError>
    where
        C: ConnectionTrait,
    {
        // Validate player name
        if player.name.trim().is_empty() {
            return Err(AppError::ChessError(
                "Player name cannot be empty".to_string(),
            ));
        }

        // Try to find an existing player with the same name
        if let Some(existing_player) = player::Entity::find()
            .filter(player::Column::Name.eq(&player.name))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to query player: {}", e)))?
        {
            // Update ELO if the new one is provided and different
            if let Some(new_elo) = player.elo {
                if existing_player.elo_rating != Some(new_elo) {
                    let mut player_model: player::ActiveModel = existing_player.clone().into();
                    player_model.elo_rating = Set(Some(new_elo));
                    player_model.updated_at = Set(Some(chrono::Utc::now()));
                    player_model.update(db).await.map_err(|e| {
                        AppError::DatabaseError(format!("Failed to update player ELO: {}", e))
                    })?;
                }
            }
            Ok(existing_player.player_id)
        } else {
            // Create new player if not found
            let player_model = player::ActiveModel {
                name: Set(player.name.clone()),
                elo_rating: Set(player.elo),
                country_code: Set(player.country.clone()),
                created_at: Set(Some(chrono::Utc::now())),
                updated_at: Set(Some(chrono::Utc::now())),
                ..Default::default()
            };
            let result = player::Entity::insert(player_model)
                .exec(db)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to create player: {}", e)))?;
            Ok(result.last_insert_id)
        }
    }

    /// Saves multiple chess games from PGN format to the database
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `pgn` - PGN string containing one or more games
    ///
    /// # Returns
    /// * `Result<Vec<Self>, AppError>` - Vector of saved games or an error
    pub async fn save_from_pgn(db: &DatabaseConnection, pgn: &str) -> Result<Vec<Self>, AppError> {
        if pgn.trim().is_empty() {
            return Err(AppError::ParseError("PGN string is empty".to_string()));
        }

        println!("Parsing PGN games...");
        let chess_games = Self::from_pgn_games(pgn)
            .map_err(|e| AppError::ParseError(format!("Failed to parse PGN: {}", e)))?;
        println!("PGN games parsed successfully: {} games", chess_games.len());

        if chess_games.is_empty() {
            return Err(AppError::ParseError("No games found in PGN".to_string()));
        }

        println!(
            "Attempting to save {} games to database...",
            chess_games.len()
        );

        let mut saved_games = Vec::new();

        // Process games in smaller batches to avoid overwhelming the database
        const BATCH_SIZE: usize = 10;
        for (batch_index, batch) in chess_games.chunks(BATCH_SIZE).enumerate() {
            println!(
                "Processing batch {} of {}",
                batch_index + 1,
                (chess_games.len() + BATCH_SIZE - 1) / BATCH_SIZE
            );

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
        // Use a transaction to ensure data consistency
        let txn = db
            .begin()
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to start transaction: {}", e)))?;

        let result = async {
            let mut game = chess_game.clone();

            // Save or find players
            game.white_player.id =
                Self::save_or_find_player(&txn, &chess_game.white_player).await?;
            game.black_player.id =
                Self::save_or_find_player(&txn, &chess_game.black_player).await?;

            // Save tournament if exists
            let tournament_id = if let Some(t) = &game.tournament {
                let tournament = tournament::ActiveModel {
                    name: Set(t.name.clone()),
                    r#type: Set(t.tournament_type.clone()),
                    time_control: Set(t.time_control.clone()),
                    start_date: Set(t.start_date.clone()),
                    end_date: Set(t.end_date.clone()),
                    location: Set(t.location.clone()),
                    ..Default::default()
                };
                let result = tournament::Entity::insert(tournament)
                    .exec(&txn)
                    .await
                    .map_err(|e| {
                        AppError::DatabaseError(format!("Failed to save tournament: {}", e))
                    })?;
                Some(result.last_insert_id)
            } else {
                None
            };

            // Save opening if exists
            let opening_id = if let Some(o) = &game.opening {
                let opening = opening::ActiveModel {
                    eco_code: Set(o.eco.clone()),
                    name: Set(o.name.clone().unwrap_or_else(|| "Unknown".to_string())),
                    variation: Set(o.variation.clone()),
                    ..Default::default()
                };
                let result = opening::Entity::insert(opening)
                    .exec(&txn)
                    .await
                    .map_err(|e| {
                        AppError::DatabaseError(format!("Failed to save opening: {}", e))
                    })?;
                Some(result.last_insert_id)
            } else {
                None
            };

            let game_date = if game.date == "????-??-??" {
                None
            } else {
                Some(game.date.clone())
            };

            // Save game
            let game_model = game::ActiveModel {
                white_player_id: Set(game.white_player.id),
                black_player_id: Set(game.black_player.id),
                tournament_id: Set(tournament_id),
                opening_id: Set(opening_id),
                result: Set(Some(game.result.clone())),
                round_number: Set(game.round),
                date_played: Set(game_date),
                fen: Set(game.fen.clone()),
                pgn: Set(game.pgn.clone().unwrap_or_else(|| "".to_string())),
                created_at: Set(Some(chrono::Utc::now())),
                ..Default::default()
            };
            let result = game::Entity::insert(game_model)
                .exec(&txn)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to save game: {}", e)))?;
            game.id = result.last_insert_id;

            // Update game_id for all moves and save them using tree structure
            // This preserves variations correctly by maintaining parent-child relationships
            let mut updated_tree = game.move_tree.clone();
            updated_tree.game_id = game.id;

            // Update game_id for all moves in the tree
            for (_, node) in updated_tree.nodes.iter_mut() {
                if let Some(ref mut chess_move) = node.game_move {
                    chess_move.game_id = game.id;
                }
            }

            // Save moves using the tree structure to preserve variations
            updated_tree
                .save_moves_to_db(&txn)
                .await
                .map_err(|e| AppError::DatabaseError(format!("Failed to save moves: {}", e)))?;

            Ok::<_, AppError>(game)
        }
        .await;

        match result {
            Ok(game) => {
                txn.commit().await.map_err(|e| {
                    AppError::DatabaseError(format!("Failed to commit transaction: {}", e))
                })?;
                Ok(game)
            }
            Err(e) => {
                let _ = txn.rollback().await; // Ignore rollback errors
                Err(e)
            }
        }
    }

    /// Converts the game to PGN format
    ///
    /// # Returns
    /// * `String` - The game in PGN format
    pub fn to_pgn(&self) -> String {
        let mut pgn = String::new();

        // Add standard tags
        pgn.push_str(&format!(
            "[Event \"{}\"]\n",
            self.tournament
                .as_ref()
                .map_or("Casual Game".to_string(), |t| t.name.clone())
        ));
        pgn.push_str(&format!(
            "[Site \"{}\"]\n",
            self.tournament
                .as_ref()
                .and_then(|t| t.location.as_ref())
                .map(String::as_str)
                .unwrap_or("?")
        ));
        pgn.push_str(&format!("[Date \"{}\"]\n", self.date));
        pgn.push_str(&format!(
            "[Round \"{}\"]\n",
            self.round.map_or("?".to_string(), |r| r.to_string())
        ));
        pgn.push_str(&format!("[White \"{}\"]\n", self.white_player.name));
        pgn.push_str(&format!("[Black \"{}\"]\n", self.black_player.name));
        pgn.push_str(&format!("[Result \"{}\"]\n", self.result));

        if let Some(ref opening) = self.opening {
            if let Some(ref eco) = opening.eco {
                pgn.push_str(&format!("[ECO \"{}\"]\n", eco));
            }
            if let Some(ref name) = opening.name {
                pgn.push_str(&format!("[Opening \"{}\"]\n", name));
            }
            if let Some(ref variation) = opening.variation {
                pgn.push_str(&format!("[Variation \"{}\"]\n", variation));
            }
        }

        if let Some(ref elo) = self.white_player.elo {
            pgn.push_str(&format!("[WhiteElo \"{}\"]\n", elo));
        }
        if let Some(ref elo) = self.black_player.elo {
            pgn.push_str(&format!("[BlackElo \"{}\"]\n", elo));
        }

        // Add any custom tags
        for tag in &self.tags {
            if !tag.starts_with("[Event ")
                && !tag.starts_with("[Site ")
                && !tag.starts_with("[Date ")
                && !tag.starts_with("[Round ")
                && !tag.starts_with("[White ")
                && !tag.starts_with("[Black ")
                && !tag.starts_with("[Result ")
                && !tag.starts_with("[ECO ")
                && !tag.starts_with("[Opening ")
                && !tag.starts_with("[Variation ")
                && !tag.starts_with("[WhiteElo ")
                && !tag.starts_with("[BlackElo ")
            {
                pgn.push_str(&format!("{}\n", tag));
            }
        }

        pgn.push('\n');

        // Add moves
        pgn.push_str(&self.move_tree.to_pgn_moves());
        pgn.push_str(&format!("{}", self.result));
        pgn
    }

    /// Makes a move in the game from the current position (as tracked in the move tree)
    ///
    /// # Arguments
    /// * `uci_move_notation` - The move in UCI notation
    ///
    /// # Returns
    /// * `Result<(), AppError>` - Success or an error
    pub async fn make_uci_move(&mut self, uci_move_notation: &str) -> Result<(), AppError> {
        if uci_move_notation.trim().is_empty() {
            return Err(AppError::ChessError(
                "Move notation cannot be empty".to_string(),
            ));
        }

        self.move_tree.make_uci_move(uci_move_notation);

        // No need to update the game as the move tree handles it
        Ok(())
    }

    /// Deletes a game from the database
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `game_id` - The ID of the game to delete
    ///
    /// # Returns
    /// * `Result<(), AppError>` - Success or an error
    pub async fn delete(db: &DatabaseConnection, game_id: i32) -> Result<(), AppError> {
        game::Entity::delete_by_id(game_id)
            .exec(db)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete game: {}", e)))?;

        Ok(())
    }

    /// Saves the game to the database
    ///
    /// # Arguments
    /// * `db` - Database connection
    ///
    /// # Returns
    /// * `Result<(), AppError>` - Success or an error
    pub async fn save(self, db: &DatabaseConnection) -> Result<Self, AppError> {
        Self::save_single_game(db, &self).await
    }
}
