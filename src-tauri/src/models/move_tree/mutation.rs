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

    /// Make a move in the move tree from the current node
    ///
    /// This simply creates a `ChessMove` and then calls `add_move`
    pub fn make_move(&mut self, move_notation: &str) {
        // Find the current node
        let current_node = self.nodes[self.current_node_id.unwrap()].clone();

        // Make the move on the current node's position
        let new_position = current_node.position.make_move(move_notation);

        if let Err(e) = new_position {
            println!("Error making move: {}", e);
            return;
        }

        // Create the move
        // FIXME: Make sure the tree is pointing to the correct node
        let mut new_move = ChessMove::from_uci(move_notation).unwrap();
        new_move.position = Some(new_position.unwrap());

        // Add the move to the move tree
        self.add_move(new_move);
    }
}
