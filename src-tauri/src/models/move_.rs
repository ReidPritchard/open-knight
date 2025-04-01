use crate::entities::{annotation, move_time_tracking, position, r#move};
use crate::parse::pgn::PgnToken;
use crate::ts_export;

use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use shakmaty::{san::San, Chess, Position};
use std::collections::HashMap;
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
        pub position: Option<ChessPosition>,
        pub annotations: Vec<ChessAnnotation>,
        pub time_info: Option<ChessMoveTime>,
        pub variations: Vec<ChessMove>,
        pub next_move: Option<Box<ChessMove>>,
    }
}

ts_export! {
    pub struct ChessPosition {
        pub id: i32,
        pub fen: String,
        pub evaluations: Vec<ChessEvaluation>,
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

impl ChessMove {
    pub async fn load_for_game(
        db: &DatabaseConnection,
        game_id: i32,
    ) -> Result<Vec<ChessMove>, Box<dyn Error>> {
        use sea_orm::ColumnTrait;
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;

        // Load all moves for the game
        let moves = r#move::Entity::find()
            .filter(r#move::Column::GameId.eq(game_id))
            .all(db)
            .await?;

        // Create a map of moves by ID for easy lookup
        let mut move_map: HashMap<i32, ChessMove> = HashMap::new();
        let mut root_moves = Vec::new();

        // First pass: create basic move objects
        for m in &moves {
            let chess_move = ChessMove {
                id: m.move_id,
                game_id: m.game_id,
                ply_number: m.ply_number,
                san: m.san.clone(),
                uci: m.uci.clone(),
                position: None,
                annotations: Vec::new(),
                time_info: None,
                variations: Vec::new(),
                next_move: None,
            };

            // Root moves are moves with ply_number 1
            if m.ply_number == 1 {
                root_moves.push(chess_move.clone());
            }
            move_map.insert(m.move_id, chess_move);
        }

        // Second pass: build the move tree
        for m in &moves {
            if m.ply_number > 1 {
                if let Some(current_move) = move_map.get(&m.move_id).cloned() {
                    if let Some(parent_move) = move_map.get_mut(&(m.move_id - 1)) {
                        parent_move.next_move = Some(Box::new(current_move));
                    }
                }
            }
        }

        Ok(root_moves)
    }

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

    pub fn from_pgn_tokens(tokens: &[PgnToken], game_id: i32) -> Vec<Self> {
        let mut moves = Vec::new();
        let mut current_number = 0i32;
        let mut is_white = true;

        for token in tokens {
            match token {
                PgnToken::MoveNumber(num) => {
                    current_number = *num as i32;
                    is_white = true;
                }
                PgnToken::Move(notation) => {
                    let chess_move = ChessMove {
                        id: 0,
                        game_id,
                        ply_number: current_number,
                        san: notation.clone(),
                        uci: notation.clone(),
                        position: None,
                        annotations: Vec::new(),
                        time_info: None,
                        variations: Vec::new(),
                        next_move: None,
                    };
                    moves.push(chess_move);
                    is_white = !is_white;
                }
                PgnToken::Variation(var_tokens) => {
                    if let Some(last_move) = moves.last_mut() {
                        last_move.variations = Self::from_pgn_tokens(var_tokens, game_id);
                    }
                }
                PgnToken::Comment(comment) => {
                    if let Some(last_move) = moves.last_mut() {
                        last_move.annotations.push(ChessAnnotation {
                            id: 0,
                            comment: Some(comment.clone()),
                            arrows: None,
                            highlights: None,
                        });
                    }
                }
                _ => {}
            }
        }

        // Link moves together
        for i in 0..moves.len() - 1 {
            moves[i].next_move = Some(Box::new(moves[i + 1].clone()));
        }

        moves
    }

    async fn generate_and_save_position(
        db: &DatabaseConnection,
        notation: &str,
        pos: &Chess,
    ) -> Result<(Option<i32>, Chess), Box<dyn Error + Send + Sync>> {
        let position_id = if let Ok(san) = notation.parse::<San>() {
            if let Ok(new_move) = san.to_move(pos) {
                if let Ok(new_pos) = pos.clone().play(&new_move) {
                    // Generate FEN for the new position
                    let fen = new_pos.board().board_fen(new_pos.promoted()).to_string();
                    let fen_hash = hash_fen(&fen);

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
                            fen: Set(fen),
                            fen_hash: Set(fen_hash),
                            created_at: Set(Some(chrono::Utc::now())),
                            ..Default::default()
                        };
                        let result = position::Entity::insert(pos_model).exec(db).await?;
                        result.last_insert_id
                    };

                    Ok((Some(position_id), new_pos))
                } else {
                    Ok((None, pos.clone()))
                }
            } else {
                Ok((None, pos.clone()))
            }
        } else {
            Ok((None, pos.clone()))
        };
        position_id
    }

    pub async fn save_moves(
        db: &DatabaseConnection,
        moves: &[ChessMove],
        starting_pos: Option<Chess>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut pos = starting_pos.unwrap_or_else(Chess::default);

        for chess_move in moves {
            // Generate and save position
            let (position_id, new_pos) =
                Self::generate_and_save_position(db, &chess_move.san, &pos).await?;
            pos = new_pos;

            // Save the move
            let move_model = r#move::ActiveModel {
                game_id: Set(chess_move.game_id),
                ply_number: Set(chess_move.ply_number),
                san: Set(chess_move.san.clone()),
                uci: Set(chess_move.uci.clone()),
                position_id: Set(position_id.unwrap_or(0)),
                created_at: Set(Some(chrono::Utc::now())),
                ..Default::default()
            };
            let result = r#move::Entity::insert(move_model).exec(db).await?;
            let current_move_id = result.last_insert_id;

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

            // Save variations recursively
            if !chess_move.variations.is_empty() {
                let variation_moves: Vec<_> = chess_move
                    .variations
                    .iter()
                    .map(|m| {
                        let mut m = m.clone();
                        m.game_id = chess_move.game_id;
                        m
                    })
                    .collect();
                Box::pin(Self::save_moves(db, &variation_moves, Some(pos.clone()))).await?;
            }
        }

        Ok(())
    }
}
