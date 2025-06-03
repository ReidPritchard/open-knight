use crate::ts_export;

use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, SlotMap};
use ts_rs::TS;

use super::{ChessMove, ChessPosition};

ts_export! {
    pub struct ChessMoveTree {
        pub game_id: i32,
        #[ts(type = "{ value: ChessTreeNode | null, version: number }[]")]
        pub nodes: SlotMap<DefaultKey, ChessTreeNode>,
        #[ts(type = "{ idx: number, version: number }", optional)]
        pub root_id: Option<DefaultKey>,
        #[ts(type = "{ idx: number, version: number }", optional)]
        pub current_node_id: Option<DefaultKey>,
    }
}

ts_export! {
    pub struct ChessTreeNode {
        pub position: ChessPosition,
        pub game_move: Option<ChessMove>, // Root node will have None
        #[ts(type = "{ idx: number, version: number } | null")]
        pub parent_id: Option<DefaultKey>,
        #[ts(type = "{ idx: number, version: number }[]")]
        pub children_ids: Vec<DefaultKey>,
    }
}

/**
 * Implement Constructors
 */
impl ChessMoveTree {
    pub fn new(game_id: i32, root_position: ChessPosition) -> Self {
        let mut tree = Self::default();
        tree.game_id = game_id;

        let root_node = ChessTreeNode {
            position: root_position,
            game_move: None,
            parent_id: None,
            children_ids: Vec::new(),
        };
        let root_node_id = tree.nodes.insert(root_node);
        tree.root_id = Some(root_node_id);
        tree.current_node_id = Some(root_node_id);

        tree
    }

    pub fn default() -> Self {
        Self {
            game_id: 0,
            nodes: SlotMap::new(),
            root_id: None,
            current_node_id: None,
        }
    }
}

mod mutation;
mod navigation;
pub mod parse;

impl ChessMoveTree {
    /// Save all moves in the tree to the database, preserving the tree structure
    /// This method correctly handles variations by setting parent_move_id based on the tree structure
    pub async fn save_moves_to_db<C>(
        &self,
        db: &C,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        C: sea_orm::ConnectionTrait,
    {
        use std::collections::HashMap;

        if let Some(root_id) = self.root_id {
            // Map from tree node IDs to database move IDs
            let mut node_to_move_id: HashMap<slotmap::DefaultKey, i32> = HashMap::new();

            // Traverse the tree and save moves in the correct order
            self.save_node_recursive(db, root_id, None, &mut node_to_move_id)
                .await?;
        }

        Ok(())
    }

    /// Recursively save a node and its children to the database
    fn save_node_recursive<'a, C>(
        &'a self,
        db: &'a C,
        node_id: slotmap::DefaultKey,
        parent_move_id: Option<i32>,
        node_to_move_id: &'a mut std::collections::HashMap<slotmap::DefaultKey, i32>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>
                + Send
                + 'a,
        >,
    >
    where
        C: sea_orm::ConnectionTrait,
    {
        Box::pin(async move {
            use crate::entities::{annotation, position, r#move};
            use sea_orm::prelude::*;
            use sea_orm::ActiveValue::Set;

            let node = &self.nodes[node_id];

            // If this node has a move (not the root), save it
            if let Some(ref chess_move) = node.game_move {
                // Save position first if it exists
                let position_id = if let Some(position) = &chess_move.position {
                    let fen = &position.fen;
                    let fen_hash = Self::hash_fen(fen);

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
                            created_at: Set(Some(sea_orm::sqlx::types::chrono::Utc::now())),
                            ..Default::default()
                        };
                        let result = position::Entity::insert(pos_model).exec(db).await?;
                        result.last_insert_id
                    };

                    Some(position_id)
                } else {
                    None
                };

                // Save the move with correct parent_move_id
                let move_model = r#move::ActiveModel {
                    game_id: Set(chess_move.game_id),
                    ply_number: Set(chess_move.ply_number),
                    san: Set(chess_move.san.clone()),
                    uci: Set(chess_move.uci.clone()),
                    position_id: Set(position_id.unwrap_or(0)),
                    parent_move_id: Set(parent_move_id),
                    created_at: Set(Some(sea_orm::sqlx::types::chrono::Utc::now())),
                    ..Default::default()
                };
                let result = r#move::Entity::insert(move_model).exec(db).await?;
                let current_move_id = result.last_insert_id;

                // Store the mapping for children to reference
                node_to_move_id.insert(node_id, current_move_id);

                // Save annotations if any
                for annotation in &chess_move.annotations {
                    if annotation.comment.is_some()
                        || annotation.arrows.is_some()
                        || annotation.highlights.is_some()
                    {
                        let anno_model = annotation::ActiveModel {
                            move_id: Set(current_move_id),
                            comment: Set(annotation.comment.clone()),
                            arrows: Set(annotation.arrows.clone()),
                            highlights: Set(annotation.highlights.clone()),
                            created_at: Set(Some(sea_orm::sqlx::types::chrono::Utc::now())),
                            ..Default::default()
                        };
                        annotation::Entity::insert(anno_model).exec(db).await?;
                    }
                }

                // Recursively save all children with this move as their parent
                for &child_id in &node.children_ids {
                    self.save_node_recursive(db, child_id, Some(current_move_id), node_to_move_id)
                        .await?;
                }
            } else {
                // This is the root node, save all its children with no parent
                for &child_id in &node.children_ids {
                    self.save_node_recursive(db, child_id, None, node_to_move_id)
                        .await?;
                }
            }

            Ok(())
        })
    }

    /// Hash a FEN string for position deduplication
    fn hash_fen(fen: &str) -> String {
        use std::hash::{DefaultHasher, Hasher};
        let mut hasher = DefaultHasher::new();
        hasher.write(fen.as_bytes());
        hasher.finish().to_string()
    }
}
