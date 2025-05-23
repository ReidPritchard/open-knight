use slotmap::DefaultKey;

use crate::models::ChessMove;

use super::{ChessMoveTree, ChessTreeNode};

impl ChessMoveTree {
    pub fn add_move(&mut self, chess_move: ChessMove) -> DefaultKey {
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

        // Return the new node ID
        new_node_id
    }

    /// Make a move in the move tree from the current node
    ///
    /// This creates a `ChessMove` and then calls `add_move`
    pub fn make_uci_move(&mut self, uci_move_notation: &str) {
        // Find the current node
        let current_node = self.nodes[self.current_node_id.unwrap()].clone();

        // Make the move on the current node's position
        let new_position = current_node.position.make_uci_move(uci_move_notation);

        if let Err(e) = new_position {
            println!("Error making move: {}", e);
            return;
        }

        // Create the ChessMove object
        let mut new_move = ChessMove::from_uci(uci_move_notation).unwrap();
        // Update the new move's properties based on where it's being added
        // position
        new_move.position = Some(new_position.unwrap());
        // ply_number
        match current_node.game_move {
            Some(ref game_move) => new_move.ply_number = game_move.ply_number + 1,
            None => new_move.ply_number = 1, // Root node - first move of the game
        }

        // Add the move to the move tree
        let new_node_id = self.add_move(new_move);

        // Move to the new node
        self.current_node_id = Some(new_node_id);
    }
}
