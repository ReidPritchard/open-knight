use slotmap::DefaultKey;

use crate::models::move_tree::ChessMoveTree;
use crate::models::ChessMove;

use super::ChessTreeNode;

impl ChessMoveTree {
    /// Navigate to the next move
    pub fn next_move(
        &mut self,
        variation_index: Option<usize>,
    ) -> bool {
        let current =
            &self.nodes[self.current_node_id.expect("Current node is not set")];

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
        let current =
            &self.nodes[self.current_node_id.expect("Current node is not set")];

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

    /// Move to a specific move by its database ID
    pub fn move_to_move(
        &mut self,
        move_db_id: i32,
    ) {
        let target_move = self.find_move(move_db_id);
        // Ignore the move node, we only need it's key to change the current node
        if let Some((key, _)) = target_move {
            self.current_node_id = Some(key);
        }
    }

    /// Move to the end of the current line
    ///
    /// If there are multiple children, the first child is always used
    /// as it's considered the "main line"
    pub fn move_to_end(&mut self) {
        if let Some(current_id) = self.current_node_id {
            let mut current_node = &self.nodes[current_id];
            while !current_node.children_ids.is_empty() {
                self.current_node_id = Some(current_node.children_ids[0]);
                current_node = &self.nodes[self.current_node_id.unwrap()];
            }
        }
    }

    /// Generates the PGN move notation with proper variation handling
    pub fn to_pgn_moves(&self) -> String {
        if let Some(root_id) = self.root_id {
            let mut pgn = String::new();
            let mut move_number = 1;
            let mut is_white = true;
            self.write_pgn_moves_recursive(
                root_id,
                &mut pgn,
                &mut move_number,
                &mut is_white,
            );
            pgn
        } else {
            String::new()
        }
    }

    fn write_pgn_moves_recursive(
        &self,
        node_id: DefaultKey,
        pgn: &mut String,
        move_number: &mut i32,
        is_white: &mut bool,
    ) {
        let node = &self.nodes[node_id];

        // Add the move if this isn't the root node
        if let Some(ref game_move) = node.game_move {
            if *is_white {
                pgn.push_str(&format!("{}. ", move_number));
            }
            pgn.push_str(&format!("{} ", game_move.san));

            if !*is_white {
                *move_number += 1;
            }
            *is_white = !*is_white;
        }

        // Process children
        if !node.children_ids.is_empty() {
            // Additional children are variations
            for &variation_id in node.children_ids.iter().skip(1) {
                pgn.push('(');

                // For variations, we need to indicate whose turn it is at the start
                let variation_node = &self.nodes[variation_id];
                if let Some(ref game_move) = variation_node.game_move {
                    // Determine whose turn it is for this variation based on ply number
                    // Odd ply numbers (1, 3, 5...) are white moves, even (2, 4, 6...) are black moves
                    let var_is_white = game_move.ply_number % 2 == 1;
                    let var_move_number = (game_move.ply_number + 1) / 2;

                    if var_is_white {
                        pgn.push_str(&format!("{}. ", var_move_number));
                    } else {
                        pgn.push_str(&format!("{}... ", var_move_number));
                    }
                    pgn.push_str(&format!("{} ", game_move.san));

                    // Continue with the variation
                    let mut var_move_num = if var_is_white {
                        var_move_number
                    } else {
                        var_move_number + 1
                    };
                    let mut var_is_white_turn = !var_is_white;

                    // Process variation's children
                    if !variation_node.children_ids.is_empty() {
                        if let Some(&child_id) =
                            variation_node.children_ids.first()
                        {
                            self.write_pgn_moves_recursive(
                                child_id,
                                pgn,
                                &mut var_move_num,
                                &mut var_is_white_turn,
                            );
                        }

                        // Handle sub-variations
                        for &sub_var_id in
                            variation_node.children_ids.iter().skip(1)
                        {
                            pgn.push('(');
                            let mut sub_var_move_num = var_move_num;
                            let mut sub_var_is_white = var_is_white_turn;
                            self.write_pgn_moves_recursive(
                                sub_var_id,
                                pgn,
                                &mut sub_var_move_num,
                                &mut sub_var_is_white,
                            );
                            pgn.push_str(") ");
                        }
                    }
                }

                pgn.push_str(") ");
            }

            // First child is the main line
            if let Some(&main_line_id) = node.children_ids.first() {
                self.write_pgn_moves_recursive(
                    main_line_id,
                    pgn,
                    move_number,
                    is_white,
                );
            }
        }
    }

    pub fn depth_first_move_traversal(
        &self
    ) -> impl Iterator<Item = ChessMove> + '_ {
        // Using a collectible approach for simplicity
        let mut moves = Vec::new();
        self.collect_moves_depth_first(
            self.root_id.expect("Root node is not set"),
            &mut moves,
        );
        moves.into_iter()
    }

    fn collect_moves_depth_first(
        &self,
        node_id: DefaultKey,
        moves: &mut Vec<ChessMove>,
    ) {
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

    /// Follow the main line of the tree
    /// Used to get the main line of positions for analysis
    /// Could also be used to get the main line of moves for a PGN export
    pub fn main_line(&self) -> Vec<ChessTreeNode> {
        let mut nodes = Vec::new();
        let mut current_id = self.current_node_id;

        while let Some(id) = current_id {
            let node = &self.nodes[id];
            nodes.push(node.clone());
            current_id = node.children_ids.first().copied();
        }
        nodes.reverse();
        nodes
    }

    /// Find a move by its ID
    ///
    /// ## Arguments
    ///
    /// * `id` - The database ID of the move to find
    ///
    /// ## Returns
    ///
    /// * `(node, key)` - The node and the key of the node if found, otherwise None
    ///
    pub fn find_move(
        &self,
        id: i32,
    ) -> Option<(DefaultKey, &ChessTreeNode)> {
        self.nodes.iter().find(|(_, node)| {
            node.game_move.as_ref().is_some_and(|m| m.id == id)
        })
    }
}
