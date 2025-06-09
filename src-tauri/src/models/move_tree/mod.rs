use crate::ts_export;

use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, SlotMap};
use ts_rs::TS;

use super::{ChessMove, ChessPosition};
use log::{debug, error, info, warn};

mod mutation;
mod navigation;
pub mod parse;

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
        let mut tree = ChessMoveTree {
            game_id: game_id,
            ..Default::default()
        };

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
}

impl Default for ChessMoveTree {
    fn default() -> Self {
        Self {
            game_id: 0,
            nodes: SlotMap::new(),
            root_id: None,
            current_node_id: None,
        }
    }
}

impl ChessMoveTree {
    /// Save all moves in the tree to the database, preserving the tree structure
    /// This method uses an iterative approach to avoid stack overflow with deep trees
    pub async fn save_moves_to_db<C>(
        &self,
        db: &C,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        C: sea_orm::ConnectionTrait,
    {
        use crate::entities::{annotation, position, r#move};
        use sea_orm::prelude::*;
        use sea_orm::ActiveValue::Set;
        use std::collections::{HashMap, VecDeque};

        info!("    → Starting move tree save (game_id: {})", self.game_id);

        if let Some(root_id) = self.root_id {
            // Map from tree node IDs to database move IDs
            let mut node_to_move_id: HashMap<slotmap::DefaultKey, i32> = HashMap::new();

            // Queue for iterative processing: (node_id, parent_move_id)
            let mut queue: VecDeque<(slotmap::DefaultKey, Option<i32>)> = VecDeque::new();

            // Start with the root node
            queue.push_back((root_id, None));

            let total_nodes = self.nodes.len();
            let mut processed_nodes = 0;

            debug!("    → Tree has {} nodes total", total_nodes);

            // Process nodes iteratively
            while let Some((node_id, parent_move_id)) = queue.pop_front() {
                // Safety check
                if !self.nodes.contains_key(node_id) {
                    warn!("    ✗ Warning: Node {:?} not found in tree", node_id);
                    continue;
                }

                let node = &self.nodes[node_id];
                processed_nodes += 1;

                // Log progress every 50 nodes for large trees
                if processed_nodes % 50 == 0 {
                    debug!(
                        "    → Progress: {}/{} nodes processed",
                        processed_nodes, total_nodes
                    );
                }

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

                    // FIXME: check if this is a variation move (if the parent node has multiple children ids)
                    // if so, we need to increment the variation number
                    let variation_order: i32 = if node.children_ids.len() > 1 {
                        node.children_ids.len() as i32
                    } else {
                        0
                    };

                    // Save the move with correct parent_move_id
                    let move_model = r#move::ActiveModel {
                        game_id: Set(chess_move.game_id),
                        ply_number: Set(chess_move.ply_number),
                        san: Set(chess_move.san.clone()),
                        uci: Set(chess_move.uci.clone()),
                        position_id: Set(position_id.unwrap_or(0)),
                        parent_move_id: Set(parent_move_id),
                        variation_order: Set(variation_order),
                        created_at: Set(Some(sea_orm::sqlx::types::chrono::Utc::now())),
                        ..Default::default()
                    };

                    let result = match r#move::Entity::insert(move_model).exec(db).await {
                        Ok(res) => res,
                        Err(e) => {
                            error!(
                                "    ✗ Error saving move at ply {}: {}",
                                chess_move.ply_number, e
                            );
                            return Err(Box::new(e));
                        }
                    };

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

                            if let Err(e) = annotation::Entity::insert(anno_model).exec(db).await {
                                warn!("    ✗ Warning: Failed to save annotation: {}", e);
                                // Continue processing, annotations are not critical
                            }
                        }
                    }

                    // Add all children to the queue with this move as their parent
                    for &child_id in &node.children_ids {
                        queue.push_back((child_id, Some(current_move_id)));
                    }
                } else {
                    // This is the root node, add all its children with no parent
                    for &child_id in &node.children_ids {
                        queue.push_back((child_id, None));
                    }
                }
            }

            debug!(
                "    → Move tree saved successfully ({} moves processed)",
                node_to_move_id.len()
            );
        } else {
            debug!("    → No root node found, skipping move tree save");
        }

        Ok(())
    }
    /// Hash a FEN string for position deduplication
    fn hash_fen(fen: &str) -> String {
        use std::hash::{DefaultHasher, Hasher};
        let mut hasher = DefaultHasher::new();
        hasher.write(fen.as_bytes());
        hasher.finish().to_string()
    }
}
