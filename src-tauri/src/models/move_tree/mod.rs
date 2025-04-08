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
