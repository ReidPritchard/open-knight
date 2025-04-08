use slotmap::DefaultKey;

use crate::models::move_tree::ChessMoveTree;
use crate::models::ChessMove;

impl ChessMoveTree {
    /// Navigate to the next move
    pub fn next_move(&mut self, variation_index: Option<usize>) -> bool {
        let current = &self.nodes[self.current_node_id.expect("Current node is not set")];

        if current.children_ids.is_empty() {
            return false;
        }

        let child_index = variation_index.unwrap_or(0);
        if child_index < current.children_ids.len() {
            self.current_node_id = Some(current.children_ids[child_index]);
            true
        } else {
            false
        }
    }

    /// Navigate to the previous move
    pub fn previous_move(&mut self) -> bool {
        let current = &self.nodes[self.current_node_id.expect("Current node is not set")];

        if let Some(parent_id) = current.parent_id {
            self.current_node_id = Some(parent_id);
            true
        } else {
            false
        }
    }

    /// Move to the root of the tree
    pub fn move_to_root(&mut self) {
        self.current_node_id = self.root_id;
    }

    pub fn depth_first_move_traversal(&self) -> impl Iterator<Item = ChessMove> + '_ {
        // Using a collectible approach for simplicity
        let mut moves = Vec::new();
        self.collect_moves_depth_first(self.root_id.expect("Root node is not set"), &mut moves);
        moves.into_iter()
    }

    fn collect_moves_depth_first(&self, node_id: DefaultKey, moves: &mut Vec<ChessMove>) {
        let node = &self.nodes[node_id];

        // Skip root node (which has no move)
        if let Some(ref game_move) = node.game_move {
            moves.push(game_move.clone());
        }

        // Recursively traverse children
        for &child_id in &node.children_ids {
            self.collect_moves_depth_first(child_id, moves);
        }
    }
}
