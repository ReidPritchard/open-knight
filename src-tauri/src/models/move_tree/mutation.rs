use crate::models::ChessMove;

use super::{ChessMoveTree, ChessTreeNode};

impl ChessMoveTree {
    pub fn add_move(&mut self, chess_move: ChessMove) {
        // Get current node's position or use the one from the move
        let position = chess_move
            .position
            .clone()
            .unwrap_or_else(|| self.nodes[self.current_node_id.unwrap()].position.clone());

        // Create the new node
        let new_node = ChessTreeNode {
            position,
            game_move: Some(chess_move),
            parent_id: Some(self.current_node_id.unwrap()),
            children_ids: Vec::new(),
        };

        // Insert into the arena and get its ID
        let new_node_id = self.nodes.insert(new_node);

        // Add as child to current node
        self.nodes[self.current_node_id.unwrap()]
            .children_ids
            .push(new_node_id);

        // Note: We don't update current_node_id here, same as original implementation
    }
}
