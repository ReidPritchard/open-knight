pub mod pgn;

use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono::{self};
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use shakmaty::uci::Uci;
use std::error::Error;
use ts_rs::TS;

use crate::entities::*;
use crate::models::{ChessMove, ChessMoveTree, ChessPosition};
use crate::parse::pgn::PgnToken;
use crate::ts_export;

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
    pub async fn new(_variant: &str, db: &DatabaseConnection) -> Result<Self, Box<dyn Error>> {
        // Create a new game struct with placeholder values
        let mut game = ChessGame {
            id: 0,
            white_player: ChessPlayer {
                id: 1,
                name: "?".to_string(),
                elo: None,
                country: None,
            },
            black_player: ChessPlayer {
                id: 2,
                name: "?".to_string(),
                elo: None,
                country: None,
            },
            tournament: None,
            opening: None,
            result: "*".to_string(),
            round: None,
            date: chrono::Utc::now().to_rfc3339(),
            move_tree: ChessMoveTree::default(),
            tags: vec!["local".to_string()],
            fen: Some(ChessPosition::default().fen),
            pgn: None,
        };

        // TODO: Init with more data

        // Insert into database

        // FIXME: Make sure this model matches the created game struct
        // we need to search for existing players and maybe the tournament
        let db_game_model = game::ActiveModel {
            white_player_id: Set(game.white_player.id.clone()),
            black_player_id: Set(game.black_player.id.clone()),
            tournament_id: Set(None),
            opening_id: Set(None),
            result: Set(Some(game.result.clone())),
            round_number: Set(game.round),
            date_played: Set(Some(game.date.clone())),
            fen: Set(game.fen.clone()),
            pgn: Set("".to_string()),
            ..Default::default()
        };

        let insert_result = game::Entity::insert(db_game_model).exec(db).await?;
        game.id = insert_result.last_insert_id;

        Ok(game)
    }

    pub async fn load(db: &DatabaseConnection, game_id: i32) -> Result<Self, Box<dyn Error>> {
        // Load the game
        let game = game::Entity::find_by_id(game_id)
            .one(db)
            .await?
            .ok_or("Game not found")?;

        // Load players
        let white_player = player::Entity::find_by_id(game.white_player_id)
            .one(db)
            .await?
            .ok_or("White player not found")?;

        let black_player = player::Entity::find_by_id(game.black_player_id)
            .one(db)
            .await?
            .ok_or("Black player not found")?;

        // Load tournament if exists
        let tournament = if let Some(tournament_id) = game.tournament_id {
            tournament::Entity::find_by_id(tournament_id)
                .one(db)
                .await?
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
                .await?
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
            result: game.result.map_or("?".to_string(), |s| s.to_string()),
            round: game.round_number,
            date: game.date_played.map_or("?".to_string(), |s| s.to_string()),
            move_tree: ChessMoveTree::default(),
            tags: Vec::new(),
            fen: game.fen,
            pgn: Some(game.pgn),
        })
    }

    pub async fn load_moves(&mut self, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
        let starting_position = ChessPosition::from_fen(self.fen.clone(), None).unwrap();
        let move_tree = load_moves_from_db(db, self.id, starting_position).await?;
        self.move_tree = move_tree;
        Ok(())
    }

    pub async fn load_tags(&mut self, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
        let tags = game_tag::Entity::find()
            .filter(game_tag::Column::GameId.eq(self.id))
            .find_also_related(tag::Entity)
            .all(db)
            .await?;

        self.tags = tags
            .into_iter()
            .filter_map(|(_, tag)| tag)
            .map(|t| format!("[{} \"{}\"]", t.name, t.description.unwrap_or_default()))
            .collect();

        Ok(())
    }

    async fn save_or_find_player(
        db: &DatabaseConnection,
        player: &ChessPlayer,
    ) -> Result<i32, Box<dyn Error>> {
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        // Try to find an existing player with the same name
        if let Some(existing_player) = player::Entity::find()
            .filter(player::Column::Name.eq(&player.name))
            .one(db)
            .await?
        {
            // Update ELO if the new one is more recent (assuming added games have more recent ELOs)
            // TODO: We should be checking if game was added more recently than the player's last known ELO
            if player.elo.is_some() {
                let mut player_model: player::ActiveModel = existing_player.clone().into();
                player_model.elo_rating = Set(player.elo);
                player_model.updated_at = Set(Some(chrono::Utc::now()));
                player_model.update(db).await?;
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
            let result = player::Entity::insert(player_model).exec(db).await?;
            Ok(result.last_insert_id)
        }
    }

    pub async fn save_from_pgn(
        db: &DatabaseConnection,
        pgn: &str,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        println!("Parsing PGN games...");
        let chess_games = Self::from_pgn_games(pgn)?;
        println!("PGN games parsed successfully");

        println!(
            "Attempting to save {} games to database...",
            chess_games.len()
        );

        // Create a vector to store the handles to our spawned tasks
        let mut handles = Vec::new();
        let db = db.clone(); // Clone the connection for use in tasks

        // Spawn a task for each game
        for chess_game in chess_games {
            let db = db.clone();

            // Save or find players synchronously
            // kinda hacky, but we want to avoid race conditions
            // that cause duplicate players to be created.
            let white_player_id = Self::save_or_find_player(&db, &chess_game.white_player).await?;
            let black_player_id = Self::save_or_find_player(&db, &chess_game.black_player).await?;

            println!("Spawning game save task #{}", handles.len());

            let handle = tokio::spawn(async move {
                let mut game = chess_game.clone();

                game.white_player.id = white_player_id;
                game.black_player.id = black_player_id;

                // Save tournament if exists
                let tournament_id = if let Some(t) = &game.tournament {
                    let tournament = tournament::ActiveModel {
                        name: Set(t.name.to_owned()),
                        r#type: Set(t.tournament_type.to_owned()),
                        time_control: Set(t.time_control.to_owned()),
                        start_date: Set(t.start_date.to_owned()),
                        end_date: Set(t.end_date.to_owned()),
                        location: Set(t.location.to_owned()),
                        ..Default::default()
                    };
                    let result = tournament::Entity::insert(tournament).exec(&db).await?;
                    Some(result.last_insert_id)
                } else {
                    None
                };

                // Save opening if exists
                let opening_id = if let Some(o) = &game.opening {
                    let opening = opening::ActiveModel {
                        eco_code: Set(o.eco.to_owned()),
                        name: Set(o.name.clone().map_or("?".to_string(), |s| s.to_owned())),
                        variation: Set(o.variation.clone()),
                        ..Default::default()
                    };
                    let result = opening::Entity::insert(opening).exec(&db).await?;
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
                    pgn: Set(game.pgn.clone().map_or("?".to_string(), |s| s.to_string())),
                    created_at: Set(Some(chrono::Utc::now())),
                    ..Default::default()
                };
                let result = game::Entity::insert(game_model).exec(&db).await?;
                game.id = result.last_insert_id;

                // Update game_id for all moves and save them
                let moves = game
                    .move_tree
                    .depth_first_move_traversal()
                    .map(|mut m| {
                        m.game_id = game.id;
                        m
                    })
                    .collect::<Vec<_>>();

                if !moves.is_empty() {
                    ChessMove::save_moves(&db, &moves).await?;
                }

                Ok::<_, Box<dyn Error + Send + Sync>>(game)
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete and collect results
        let mut saved_games = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(game)) => saved_games.push(game),
                Ok(Err(e)) => eprintln!("Error saving game: {}", e),
                Err(e) => eprintln!("Task panicked: {}", e),
            }
        }

        Ok(saved_games)
    }

    pub fn to_pgn(&mut self) -> String {
        let mut pgn = String::new();

        // Add standard tags
        pgn.push_str(&format!(
            "[Event \"{}\"]\n",
            self.tournament
                .as_ref()
                .map_or("?".to_string(), |t| t.name.clone())
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

        // Add moves (this is a basic implementation, you might want to enhance it)
        let mut move_number = 1;
        let mut is_white = true;
        for chess_move in self.move_tree.depth_first_move_traversal() {
            if is_white {
                pgn.push_str(&format!("{}. ", move_number));
            }
            pgn.push_str(&format!("{} ", chess_move.san));

            if !is_white {
                move_number += 1;
            }
            is_white = !is_white;
        }

        pgn.push_str(&format!("{}", self.result));
        pgn
    }

    pub async fn make_move(
        &mut self,
        move_notation: &str,
        current_move_id: i32,
    ) -> Result<Self, Box<dyn Error>> {
        let mut move_tree = self.move_tree.clone();

        // FIXME: Implement a better way to find the move we want to add the new move to

        // Goto the `current_move_id` and add the new move
        let current_move = move_tree.find_move(current_move_id).unwrap();

        // Make the move
        let new_position = current_move.position.make_move(move_notation)?;

        // Update the game
        self.fen = Some(new_position.fen);

        // Update the move tree
        move_tree.make_move(move_notation);
        self.move_tree = move_tree;

        Ok(self.clone())
    }
}
