use std::{cell::RefCell, rc::Rc};

use crate::models::{APIMove, Move, Position};

#[derive(Debug)]
pub struct ChessMoveTree {
    pub game_id: i32,
    pub root: Rc<RefCell<ChessTreeNode>>,
    pub current_node: Rc<RefCell<ChessTreeNode>>,
}

#[derive(Debug, Clone)]
pub struct ChessTreeNode {
    pub position: Position,
    pub game_move: Option<Move>, // Root node will have None
    pub parent: Option<Rc<RefCell<ChessTreeNode>>>,
    pub children: Vec<Rc<RefCell<ChessTreeNode>>>,
}

impl ChessTreeNode {
    pub fn new(position: Position, game_move: Option<Move>) -> Self {
        Self {
            position,
            game_move,
            parent: None,
            children: vec![],
        }
    }
}

impl ChessMoveTree {
    pub fn new(game_id: i32, root_position: Position) -> Self {
        let root = ChessTreeNode::new(root_position, None);
        Self {
            game_id,
            root: Rc::new(RefCell::new(root.clone())),
            current_node: Rc::new(RefCell::new(root.clone())),
        }
    }
}
