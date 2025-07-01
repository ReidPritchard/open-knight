use crate::models::{ChessMoveTree, ChessPosition};

impl ChessMoveTree {
    /// Extract all positions from the move tree
    ///
    /// Parameters:
    /// - `include_variations`: Whether to include variations in the extraction
    ///
    /// Returns a vector of ChessPosition objects
    pub fn extract_positions(&self, include_variations: bool) -> Vec<ChessPosition> {
        let mut positions: Vec<ChessPosition> = Vec::new();

        let tree_nodes = if include_variations {
            self.nodes.values().cloned().collect()
        } else {
            self.main_line()
        };

        for node in tree_nodes {
            positions.push(node.position.clone());
        }

        positions
    }
}
