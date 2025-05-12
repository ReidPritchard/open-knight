use std::error::Error;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use shakmaty::Chess;
use slotmap::{DefaultKey, SlotMap};

use crate::entities::{position, r#move};
use crate::models::{generate_uci, ChessAnnotation, ChessMove, ChessPosition};
use crate::parse::pgn::PgnToken;

use super::{ChessMoveTree, ChessTreeNode};

/**
 * Given a list of PGN move related tokens, parse them into a ChessMoveTree
 * @param tokens - The list of PGN move related tokens
 * @param game_id - The ID of the game
 * @param root_position - The starting position of the game
 * @returns the parsed ChessMoveTree
 */
pub fn parse_pgn_tokens(
    game_id: i32,
    root_position: ChessPosition,
    tokens: &[PgnToken],
) -> Result<ChessMoveTree, Box<dyn Error>> {
    // Create a new tree with just the root node
    let mut nodes = SlotMap::new();

    // Create the root node
    let root_node = ChessTreeNode {
        position: root_position.clone(),
        game_move: None,
        parent_id: None,
        children_ids: Vec::new(),
    };

    let root_id = nodes.insert(root_node);

    let mut tree = ChessMoveTree {
        game_id,
        nodes,
        root_id: Some(root_id),
        current_node_id: Some(root_id),
    };

    // Initialize the parsing state
    let current_position = root_position;
    let mut full_move_count = 0;
    let mut is_white = true;

    // Process tokens recursively
    parse_pgn_tokens_recursive(
        tokens,
        &mut tree,
        current_position,
        root_id,
        &mut full_move_count,
        &mut is_white,
    )?;

    Ok(tree)
}

/**
 * A recursive function for parse_pgn_tokens
 */
fn parse_pgn_tokens_recursive(
    tokens: &[PgnToken],
    tree: &mut ChessMoveTree,
    mut current_position: ChessPosition,
    current_node_id: DefaultKey,
    full_move_count: &mut i32,
    is_white: &mut bool,
) -> Result<(), Box<dyn Error>> {
    let mut current_node_id = current_node_id;

    for token in tokens {
        match token {
            PgnToken::MoveNumber(num) => {
                let new_move_count = *num as i32;
                // If the new move count is different from the previous move count, we have a new full move
                // therefore the next move is white, if it's the same, the next move is black
                // (ex. 1. e4 1... e5 2... d4 3... d5 4...)
                *is_white = new_move_count != *full_move_count;
                *full_move_count = new_move_count;
            }
            PgnToken::Move(notation) => {
                // ply number
                let new_move_ply = *full_move_count * 2 + if *is_white { -1 } else { 0 };

                // post move position
                let new_move_position = current_position.make_move(&notation)?;

                // UCI
                let uci = generate_uci(notation, &Chess::from(current_position.clone()))?;

                let new_move = ChessMove {
                    id: 0,
                    game_id: tree.game_id,
                    ply_number: new_move_ply,
                    san: notation.clone(),
                    uci,
                    position: Some(new_move_position.clone()),
                    annotations: Vec::new(),
                    time_info: None,
                    parent_move_id: None, // Make sure to set this once the moves have ids
                };

                // Create a new node for the move
                let new_move_node = ChessTreeNode {
                    position: new_move_position.clone(),
                    game_move: Some(new_move),
                    parent_id: Some(current_node_id),
                    children_ids: Vec::new(),
                };

                // Insert the new node and get its ID
                let new_node_id = tree.nodes.insert(new_move_node);

                // Add the new move node to the current node's children
                tree.nodes[current_node_id].children_ids.push(new_node_id);

                // Update the current node and position
                current_node_id = new_node_id;
                current_position = new_move_position;

                *is_white = !*is_white;
            }
            PgnToken::Variation(var_tokens) => {
                // Save the current state before processing the variation
                let saved_position = current_position.clone();
                let saved_node_id = current_node_id;
                let saved_move_count = *full_move_count;
                let saved_is_white = *is_white;

                // Process the variation recursively
                parse_pgn_tokens_recursive(
                    var_tokens,
                    tree,
                    saved_position,
                    saved_node_id,
                    &mut saved_move_count.clone(),
                    &mut saved_is_white.clone(),
                )?;
            }
            PgnToken::Comment(comment) => {
                // If we're at a move node (not the root), add the comment to the move
                if let Some(ref mut game_move) = tree.nodes[current_node_id].game_move.as_mut() {
                    game_move.annotations.push(ChessAnnotation {
                        id: 0,
                        comment: Some(comment.clone()),
                        arrows: None,
                        highlights: None,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/**
 * Given a game ID, load the moves from the database and parse them into a ChessMoveTree
 * @param game_id - The ID of the game
 * @param root_position - The starting position of the game
 * @returns the parsed ChessMoveTree
 */
pub async fn load_moves_from_db(
    db: &DatabaseConnection,
    game_id: i32,
    root_position: ChessPosition,
) -> Result<ChessMoveTree, Box<dyn Error>> {
    let all_db_moves = r#move::Entity::find()
        .filter(r#move::Column::GameId.eq(game_id))
        .all(db)
        .await?;
    let move_positions = all_db_moves.load_one(position::Entity, db).await?;

    // But now we can construct the tree with arena allocation
    let mut nodes = SlotMap::new();

    // Create root node
    let root_node = ChessTreeNode {
        position: root_position,
        game_move: None,
        parent_id: None,
        children_ids: Vec::new(),
    };

    let root_id = nodes.insert(root_node);
    let mut current_id = root_id;

    // Build the tree from loaded moves...
    // [Implementation details]
    for (move_entity, position_entity) in all_db_moves.into_iter().zip(move_positions.into_iter()) {
        let move_position = position_entity.map_or(
            ChessPosition {
                id: 0,
                fen: "".to_string(),
                evaluations: Vec::new(),
                variant: None,
            },
            |p| ChessPosition {
                id: p.position_id,
                fen: p.fen,
                evaluations: Vec::new(),
                variant: None,
            },
        );

        let new_move_node = ChessTreeNode {
            position: move_position.clone(),
            game_move: Some(ChessMove {
                id: move_entity.move_id,
                game_id,
                ply_number: move_entity.ply_number,
                san: move_entity.san,
                uci: move_entity.uci,
                position: Some(move_position),
                annotations: Vec::new(),
                time_info: None,
                parent_move_id: move_entity.parent_move_id.clone(),
            }),
            parent_id: Some(current_id),
            children_ids: Vec::new(),
        };

        let new_id = nodes.insert(new_move_node);
        nodes[current_id].children_ids.push(new_id);
        current_id = new_id;
    }

    Ok(ChessMoveTree {
        game_id,
        nodes,
        root_id: Some(root_id),
        current_node_id: Some(root_id), // Or last position in the main line
    })
}
