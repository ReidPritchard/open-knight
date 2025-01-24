use crate::entities::{annotation, move_, move_time_tracking, position};
use crate::parse::pgn::PgnToken;
use crate::ts_export;

use sea_orm::sqlx::types::chrono;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use shakmaty::{san::San, Chess, Position};
use std::collections::HashMap;
use std::error::Error;
use ts_rs::TS;

ts_export! {
    pub struct ChessMove {
        pub id: i32,
        pub game_id: i32,
        pub move_number: Option<i32>,
        pub player_color: Option<String>,
        pub notation: String,
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

impl ChessMove {
    pub async fn load_for_game(
        db: &DatabaseConnection,
        game_id: i32,
    ) -> Result<Vec<ChessMove>, Box<dyn Error>> {
        use sea_orm::ColumnTrait;
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;

        // Load all moves for the game
        let moves = move_::Entity::find()
            .filter(move_::Column::GameId.eq(game_id))
            .all(db)
            .await?;

        // Create a map of moves by ID for easy lookup
        let mut move_map: HashMap<i32, ChessMove> = HashMap::new();
        let mut root_moves = Vec::new();

        // First pass: create basic move objects
        for m in &moves {
            let chess_move = ChessMove {
                id: m.id,
                game_id: m.game_id,
                move_number: m.move_number,
                player_color: m.player_color.clone(),
                notation: m.move_notation.clone(),
                position: None,
                annotations: Vec::new(),
                time_info: None,
                variations: Vec::new(),
                next_move: None,
            };

            if m.parent_move_id.is_none() {
                root_moves.push(chess_move.clone());
            }
            move_map.insert(m.id, chess_move);
        }

        // Second pass: build the move tree
        for m in &moves {
            if let Some(parent_id) = m.parent_move_id {
                if let Some(current_move) = move_map.get(&m.id).cloned() {
                    if let Some(parent_move) = move_map.get_mut(&parent_id) {
                        parent_move.variations.push(current_move);
                    }
                }
            }
        }

        // Sort moves by move number and build the main line
        root_moves.sort_by_key(|m| m.move_number);

        // Build the linked list of main line moves
        for i in 0..root_moves.len() - 1 {
            root_moves[i].next_move = Some(Box::new(root_moves[i + 1].clone()));
        }

        Ok(root_moves)
    }

    pub async fn load_position(&mut self, db: &DatabaseConnection) -> Result<(), Box<dyn Error>> {
        use sea_orm::EntityTrait;

        let move_data = move_::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .ok_or("Move not found")?;

        if let Some(position_id) = move_data.position_id {
            let pos = position::Entity::find_by_id(position_id)
                .one(db)
                .await?
                .ok_or("Position not found")?;

            self.position = Some(ChessPosition {
                id: pos.id,
                fen: pos.fen,
                evaluations: Vec::new(), // TODO: Load evaluations
            });
        }

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
                id: a.id,
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
                    println!("Move number: {}", num);
                    current_number = *num as i32;
                    is_white = true;
                }
                PgnToken::Move(notation) => {
                    let chess_move = ChessMove {
                        id: 0,
                        game_id,
                        move_number: Some(current_number),
                        player_color: Some(if is_white {
                            "white".to_string()
                        } else {
                            "black".to_string()
                        }),
                        notation: notation.clone(),
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

                    // Check if position already exists
                    use sea_orm::ColumnTrait;
                    use sea_orm::QueryFilter;
                    let existing_position = position::Entity::find()
                        .filter(position::Column::Fen.eq(&fen))
                        .one(db)
                        .await?;

                    let position_id = if let Some(existing_pos) = existing_position {
                        existing_pos.id
                    } else {
                        // Save new position if it doesn't exist
                        let pos_model = position::ActiveModel {
                            fen: Set(fen),
                            created_at: Set(chrono::Utc::now().to_rfc3339()),
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
        let mut last_move_id = None;
        let mut last_move_color = None;
        let mut pos = starting_pos.unwrap_or_else(Chess::default);

        for chess_move in moves {
            // Generate and save position
            let (position_id, new_pos) =
                Self::generate_and_save_position(db, &chess_move.notation, &pos).await?;
            pos = new_pos;

            // Determine the player color based on the previous move
            // If previous move was white, then the current move is black
            let player_color = if last_move_id.is_some() {
                if last_move_color == Some("white".to_string()) {
                    "black".to_string()
                } else {
                    "white".to_string()
                }
            } else {
                "white".to_string()
            };

            last_move_color = Some(player_color.clone());

            // Save the move
            let move_model = move_::ActiveModel {
                game_id: Set(chess_move.game_id),
                move_number: Set(chess_move.move_number),
                player_color: Set(Some(player_color)),
                move_notation: Set(chess_move.notation.clone()),
                parent_move_id: Set(last_move_id),
                position_id: Set(position_id),
                created_at: Set(chrono::Utc::now().to_rfc3339()),
                ..Default::default()
            };
            let result = move_::Entity::insert(move_model).exec(db).await?;
            let current_move_id = result.last_insert_id;
            last_move_id = Some(current_move_id);

            // Save annotations if any
            for annotation in &chess_move.annotations {
                if let Some(ref comment) = annotation.comment {
                    let anno_model = annotation::ActiveModel {
                        move_id: Set(current_move_id),
                        comment: Set(Some(comment.clone())),
                        arrows: Set(annotation.arrows.clone()),
                        highlights: Set(annotation.highlights.clone()),
                        created_at: Set(chrono::Utc::now().to_rfc3339()),
                        ..Default::default()
                    };
                    annotation::Entity::insert(anno_model).exec(db).await?;
                }
            }

            // Save variations with the current move as their parent
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
                // Pass the current position for the variation
                Box::pin(Self::save_moves(db, &variation_moves, Some(pos.clone()))).await?;
            }
        }

        Ok(())
    }
}
