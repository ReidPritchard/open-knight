use crate::entities::{annotation, move_time_tracking, position, r#move};
use crate::ts_export;

use sea_orm::prelude::*;
use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{ConnectionTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{san::San, Chess, Position};
use shakmaty::{CastlingMode, EnPassantMode, Move};
use std::error::Error;
use std::hash::{DefaultHasher, Hasher};
use ts_rs::TS;

ts_export! {
    pub struct ChessMove {
        pub id: i32,
        pub game_id: i32,
        pub ply_number: i32,
        pub san: String,
        pub uci: String,
        pub position: Option<ChessPosition>, // The resulting position after the move is played
        pub annotations: Vec<ChessAnnotation>,
        pub time_info: Option<ChessMoveTime>,
        pub parent_move_id: Option<i32>,
    }
}

ts_export! {
    pub struct ChessPosition {
        pub id: i32,
        pub fen: String,
        pub evaluations: Vec<ChessEvaluation>,
        pub variant: Option<String>, // TODO: Handle Chess960
    }
}

ts_export! {
    pub struct ChessAnnotation {
        pub id: i32,
        pub comment: Option<String>,
        pub arrows: Option<String>,
        pub highlights: Option<String>,
    }
}

ts_export! {
    pub struct ChessMoveTime {
        pub time_spent_ms: Option<i32>,
        pub time_left_ms: Option<i32>,
    }
}

ts_export! {
    pub struct ChessEvaluation {
        pub score: Option<f32>,
        pub eval_type: Option<String>,
        pub depth: Option<i32>,
        pub engine: Option<String>,
    }
}

fn hash_fen(fen: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(fen.as_bytes());
    hasher.finish().to_string()
}

pub fn generate_uci(san: &str, pos: &Chess) -> Result<String, Box<dyn Error>> {
    let uci = san
        .parse::<San>()
        .unwrap()
        .to_move(pos)
        .unwrap()
        .to_uci(CastlingMode::Standard)
        .to_string();
    Ok(uci)
}

// Convert a ChessPosition to a shakmaty::Position
impl From<ChessPosition> for Chess {
    fn from(position: ChessPosition) -> Self {
        let fen: Fen = position.fen.parse::<Fen>().unwrap();
        let variant: CastlingMode = match position.variant {
            Some(variant) => match variant.as_str() {
                "Chess960" => CastlingMode::Chess960,
                "Standard" => CastlingMode::Standard,
                _ => CastlingMode::Standard,
            },
            None => CastlingMode::Standard,
        };
        fen.into_position(variant).unwrap()
    }
}

impl ChessPosition {
    pub fn from_fen(fen: Option<String>, variant: Option<String>) -> Result<Self, Box<dyn Error>> {
        let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let default_variant = "Standard";

        let fen_str = fen.unwrap_or(default_fen.to_string());
        let variant_str = variant.unwrap_or(default_variant.to_string());

        Ok(ChessPosition {
            id: 0,
            fen: fen_str,
            evaluations: Vec::new(),
            variant: Some(variant_str),
        })
    }

    pub fn make_san_move(&self, move_san: &str) -> Result<Self, Box<dyn Error>> {
        let pos = Chess::from(self.clone());
        let parsed_move = San::from_ascii(move_san.as_bytes())?;
        let chess_move = parsed_move.to_move(&pos)?;
        let new_pos = pos.play(&chess_move)?;
        let fen = Fen::from_position(new_pos, EnPassantMode::Legal).to_string();
        Ok(ChessPosition {
            id: self.id,
            fen,
            evaluations: Vec::new(),
            variant: self.variant.clone(),
        })
    }

    /// Make a move from a UCI notation string
    pub fn make_uci_move(&self, uci: &str) -> Result<(Self, Move), Box<dyn Error>> {
        let pos = Chess::from(self.clone());
        let parsed_move = UciMove::from_ascii(uci.as_bytes())?;
        let chess_move = parsed_move.to_move(&pos)?;
        let new_pos = pos.play(&chess_move)?;
        let fen = Fen::from_position(new_pos, EnPassantMode::Legal).to_string();
        Ok((
            ChessPosition {
                id: self.id,
                fen,
                evaluations: Vec::new(),
                variant: self.variant.clone(),
            },
            chess_move,
        ))
    }

    /**
     * Default constructor
     * Creates a new position with the starting position of a standard chess game
     */
    pub fn default() -> Self {
        Self::from_fen(
            Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
            None,
        )
        .unwrap()
    }
}

impl ChessMove {
    pub async fn load_position(&mut self, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
        use sea_orm::EntityTrait;

        let move_data = r#move::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .ok_or("Move not found")?;

        let position_id = move_data.position_id;
        let pos = position::Entity::find_by_id(position_id)
            .one(db)
            .await?
            .ok_or("Position not found")?;

        self.position = Some(ChessPosition {
            id: pos.position_id,
            fen: pos.fen,
            evaluations: Vec::new(), // TODO: Load evaluations
            variant: None,           // TODO: Load variant
        });

        Ok(())
    }

    pub async fn load_annotations(
        &mut self,
        db: &DatabaseConnection,
    ) -> Result<(), Box<dyn Error>> {
        use sea_orm::ColumnTrait;
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;

        let annotations = annotation::Entity::find()
            .filter(annotation::Column::MoveId.eq(self.id))
            .all(db)
            .await?;

        self.annotations = annotations
            .into_iter()
            .map(|a| ChessAnnotation {
                id: a.annotation_id,
                comment: a.comment,
                arrows: a.arrows,
                highlights: a.highlights,
            })
            .collect();

        Ok(())
    }

    pub async fn load_time_info(&mut self, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
        use sea_orm::ColumnTrait;
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;

        let time_info = move_time_tracking::Entity::find()
            .filter(move_time_tracking::Column::MoveId.eq(self.id))
            .one(db)
            .await?;

        if let Some(time) = time_info {
            self.time_info = Some(ChessMoveTime {
                time_spent_ms: time.time_spent_ms,
                time_left_ms: time.time_left_ms,
            });
        }

        Ok(())
    }

    pub async fn save_moves<C>(
        db: &C,
        moves: &[ChessMove],
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        C: ConnectionTrait,
    {
        // FIXME: I think this doesn't handle variations correctly since multiple moves can have the same parent move id
        // and this assumes that all moves are the children of the previous move
        let mut parent_move_id = None;

        for chess_move in moves {
            // Get position_id from the move's position
            let position_id = if let Some(position) = &chess_move.position {
                let fen = &position.fen;
                let fen_hash = hash_fen(fen);

                // Check if position already exists
                use sea_orm::ColumnTrait;
                use sea_orm::QueryFilter;
                let existing_position = position::Entity::find()
                    .filter(position::Column::FenHash.eq(&fen_hash))
                    .one(db)
                    .await?;

                let position_id = if let Some(existing_pos) = existing_position {
                    existing_pos.position_id
                } else {
                    // Save new position if it doesn't exist
                    let pos_model = position::ActiveModel {
                        fen: Set(fen.clone()),
                        fen_hash: Set(fen_hash),
                        created_at: Set(Some(chrono::Utc::now())),
                        ..Default::default()
                    };
                    let result = position::Entity::insert(pos_model).exec(db).await?;
                    result.last_insert_id
                };

                Some(position_id)
            } else {
                None
            };

            // Save the move
            let move_model = r#move::ActiveModel {
                game_id: Set(chess_move.game_id),
                ply_number: Set(chess_move.ply_number),
                san: Set(chess_move.san.clone()),
                uci: Set(chess_move.uci.clone()),
                position_id: Set(position_id.unwrap_or(0)),
                parent_move_id: Set(parent_move_id),
                created_at: Set(Some(chrono::Utc::now())),
                ..Default::default()
            };
            let result = r#move::Entity::insert(move_model).exec(db).await?;
            let current_move_id = result.last_insert_id;
            parent_move_id = Some(current_move_id);

            // Save annotations if any
            for annotation in &chess_move.annotations {
                if let Some(ref comment) = annotation.comment {
                    let anno_model = annotation::ActiveModel {
                        move_id: Set(current_move_id),
                        comment: Set(Some(comment.clone())),
                        arrows: Set(annotation.arrows.clone()),
                        highlights: Set(annotation.highlights.clone()),
                        created_at: Set(Some(chrono::Utc::now())),
                        ..Default::default()
                    };
                    annotation::Entity::insert(anno_model).exec(db).await?;
                }
            }
        }

        Ok(())
    }

    /**
     * Convert a Move DB Entity into a ChessMove
     * @param move_data - The Move DB Entity
     * @param position_data - The Position DB Entity
     * @returns the converted ChessMove
     */
    pub fn from_db_entities(
        move_data: r#move::Model,
        position_data: Option<position::Model>,
        game_variant: Option<String>,
    ) -> Self {
        let move_variant = game_variant.unwrap_or("Standard".to_string());
        let move_position = position_data.map_or(
            ChessPosition {
                id: 0,
                fen: "".to_string(),
                evaluations: Vec::new(),
                variant: Some(move_variant.clone()),
            },
            |p| ChessPosition {
                id: p.position_id,
                fen: p.fen,
                evaluations: Vec::new(),
                variant: Some(move_variant.clone()),
            },
        );

        ChessMove {
            id: move_data.move_id,
            game_id: move_data.game_id,
            ply_number: move_data.ply_number,
            san: move_data.san,
            uci: move_data.uci,
            position: Some(move_position),
            annotations: Vec::new(),
            time_info: None,
            parent_move_id: move_data.parent_move_id,
        }
    }

    /// Create a ChessMove from a UCI notation string
    ///
    /// This is a convenience function that creates a ChessMove from a UCI notation string.
    /// It does not load the position from the database, so the position will be None.
    /// It also does not correctly generate the SAN notation, so the SAN will be the UCI string!!
    ///
    /// @param uci - The UCI notation string
    /// @returns the converted ChessMove
    pub fn from_uci(uci: &str) -> Result<Self, Box<dyn Error>> {
        let move_san = San::from_ascii(uci.as_bytes())?; // FIXME: This is not correct
        let chess_move = ChessMove {
            id: 0,
            uci: uci.to_string(),
            game_id: 0,
            ply_number: 0,
            san: move_san.to_string(),
            position: None,
            annotations: Vec::new(),
            time_info: None,
            parent_move_id: None,
        };

        Ok(chess_move)
    }
}
