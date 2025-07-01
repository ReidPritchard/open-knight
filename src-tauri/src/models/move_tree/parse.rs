use std::collections::HashMap;
use std::error::Error;

use itertools::izip;
use log::{debug, info};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter,
};
use shakmaty::Chess;
use slotmap::{DefaultKey, SlotMap};

use crate::entities::{annotation, position, r#move};
use crate::models::{generate_uci, ChessAnnotation, ChessMove, ChessPosition};
use ok_parse::pgn::PgnToken;

use super::{ChessMoveTree, ChessTreeNode};

/**
 * Given a list of PGN move related tokens, parse them into a ChessMoveTree
 * @param tokens - The list of PGN move related tokens
 * @param game_id - The ID of the game
 * @param root_position - The starting position of the game
 * @returns the parsed ChessMoveTree
 */
pub fn pgn_tokens_to_move_tree(
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
            PgnToken::MoveNumber { number } => {
                let new_move_count = *number as i32;
                // If the new move count is different from the previous move count, we have a new full move
                // therefore the next move is white, if it's the same, the next move is black
                // (ex. 1. e4 1... e5 2... d4 3... d5 4...)
                *is_white = new_move_count != *full_move_count;
                *full_move_count = new_move_count;
            }
            PgnToken::Move { notation } => {
                // ply number
                let new_move_ply =
                    *full_move_count * 2 + if *is_white { -1 } else { 0 };

                // post move position
                let new_move_position_result =
                    current_position.make_san_move(notation);
                if let Err(e) = new_move_position_result {
                    eprintln!(
                        "Error making move.\nNotation: {}\nError:\n{}",
                        notation, e
                    );
                    return Err(e);
                }

                let new_move_position = new_move_position_result.unwrap();

                // UCI
                let uci = generate_uci(
                    notation,
                    &Chess::from(current_position.clone()),
                )?;

                let new_move = ChessMove {
                    id: 0,
                    game_id: tree.game_id,
                    ply_number: new_move_ply,
                    san: notation.clone(),
                    uci,
                    position: Some(new_move_position.clone()),
                    annotations: Vec::new(),
                    time_info: None,
                    parent_move_id: None, // Make sure to set this once the moves have database ids
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
            PgnToken::Variation { moves: var_tokens } => {
                // Per PGN spec: RAV is played by "first unplaying the move that appears immediately prior to the RAV"
                // This means variations branch from the parent position, not the current position
                let parent_node_id = tree.nodes[current_node_id].parent_id;
                let (parent_position, parent_move_count, parent_is_white) =
                    if let Some(parent_id) = parent_node_id {
                        let parent_node = &tree.nodes[parent_id];
                        let parent_position = parent_node.position.clone();

                        // Calculate the move count and turn at the parent position
                        if let Some(ref parent_move) = parent_node.game_move {
                            // Parent has a move, so calculate state after that move
                            let move_count = (parent_move.ply_number + 1) / 2;
                            let is_white = parent_move.ply_number % 2 == 0; // Next turn after this ply
                            (parent_position, move_count, is_white)
                        } else {
                            // Parent is root node
                            (parent_position, 1, true)
                        }
                    } else {
                        // Current node is root, use current state
                        (current_position.clone(), *full_move_count, *is_white)
                    };

                // Save the current state before processing the variation
                let saved_position = current_position.clone();
                let saved_node_id = current_node_id;
                let saved_move_count = *full_move_count;
                let saved_is_white = *is_white;

                // Process the variation recursively from the parent position
                parse_pgn_tokens_recursive(
                    var_tokens,
                    tree,
                    parent_position,
                    parent_node_id.unwrap_or(current_node_id),
                    &mut parent_move_count.clone(),
                    &mut parent_is_white.clone(),
                )?;

                // Restore the current state after processing the variation
                current_node_id = saved_node_id;
                current_position = saved_position;
                *full_move_count = saved_move_count;
                *is_white = saved_is_white;
            }
            PgnToken::Comment { text } => {
                // If we're at a move node (not the root), add the comment to the move
                if let Some(ref mut game_move) =
                    tree.nodes[current_node_id].game_move.as_mut()
                {
                    game_move.annotations.push(ChessAnnotation {
                        id: 0,
                        comment: Some(text.clone()),
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
    info!("Loading moves from database for game {}", game_id);

    let all_db_moves = r#move::Entity::find()
        .filter(r#move::Column::GameId.eq(game_id))
        .all(db)
        .await?;
    let move_positions = all_db_moves.load_one(position::Entity, db).await?;
    let annotations = all_db_moves.load_many(annotation::Entity, db).await?;

    debug!("Found {} moves in database", all_db_moves.len());

    let mut nodes = SlotMap::new();
    let mut db_id_to_key: HashMap<i32, DefaultKey> = HashMap::new();

    // Create root node
    let root_node = ChessTreeNode {
        position: root_position,
        game_move: None,
        parent_id: None,
        children_ids: Vec::new(),
    };

    let root_id = nodes.insert(root_node);

    let data = izip!(all_db_moves.clone(), move_positions, annotations);

    debug!("Starting first pass to create move nodes");

    // First pass: Create all move nodes and build the ID mapping
    for (move_entity, position_entity, annotation_entities) in data {
        let move_position = position_entity.as_ref().map_or(
            ChessPosition {
                id: 0,
                fen: "".to_string(),
                evaluations: Vec::new(),
                variant: None,
            },
            |p| ChessPosition {
                id: p.position_id,
                fen: p.fen.clone(),
                evaluations: Vec::new(),
                variant: None,
            },
        );

        let annotations = annotation_entities
            .iter()
            .map(|a| ChessAnnotation {
                id: a.annotation_id,
                comment: a.comment.clone(),
                arrows: a.arrows.clone(),
                highlights: a.highlights.clone(),
            })
            .collect::<Vec<_>>();

        let new_move_node = ChessTreeNode {
            position: move_position.clone(),
            game_move: Some(ChessMove {
                id: move_entity.move_id,
                game_id,
                ply_number: move_entity.ply_number,
                san: move_entity.san.clone(),
                uci: move_entity.uci.clone(),
                position: Some(move_position),
                annotations,
                time_info: None,
                parent_move_id: move_entity.parent_move_id,
            }),
            parent_id: None, // Will be set in second pass
            children_ids: Vec::new(),
        };

        let new_id = nodes.insert(new_move_node);
        db_id_to_key.insert(move_entity.move_id, new_id);
    }

    debug!("Finished first pass to create move nodes");
    debug!("Starting second pass to establish parent-child relationships");

    // Second pass: Establish parent-child relationships
    for move_entity in &all_db_moves {
        let current_key = db_id_to_key[&move_entity.move_id];

        // Set the parent_id for the current node
        let parent_key = match move_entity.parent_move_id {
            None => root_id, // Move has no parent, so it's a child of root
            Some(parent_db_id) => {
                // Find the SlotMap key for the parent move
                *db_id_to_key
                    .get(&parent_db_id)
                    .ok_or("Parent move not found")?
            }
        };
        nodes[current_key].parent_id = Some(parent_key);

        // Add current node to parent's children
        nodes[parent_key].children_ids.push(current_key);
    }

    info!(
        "Loaded {} moves from database for game {}",
        all_db_moves.len(),
        game_id
    );

    Ok(ChessMoveTree {
        game_id,
        nodes,
        root_id: Some(root_id),
        current_node_id: Some(root_id),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pgn_tokens_simple_game() {
        let tokens = vec![
            PgnToken::MoveNumber { number: 1 },
            PgnToken::Move {
                notation: "e4".to_string(),
            },
            PgnToken::Move {
                notation: "e5".to_string(),
            },
            PgnToken::MoveNumber { number: 2 },
            PgnToken::Move {
                notation: "d4".to_string(),
            },
            PgnToken::Move {
                notation: "d5".to_string(),
            },
        ];

        let mut tree =
            pgn_tokens_to_move_tree(1, ChessPosition::default(), &tokens)
                .unwrap();
        assert_eq!(tree.nodes.len(), 5);
        // let serialized = serde_json::to_string(&tree).unwrap();
        // println!("{}", serialized);
        // assert_eq!(serialized, "{}");

        let first_node = &tree.nodes[tree.current_node_id.unwrap()];
        // Start position
        assert_eq!(
            first_node.position.fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        // No move
        assert!(first_node.game_move.is_none());

        let second_node = &tree.nodes[first_node.children_ids[0]];
        // e4
        assert_eq!(second_node.game_move.as_ref().unwrap().san, "e4");
        assert_eq!(second_node.game_move.as_ref().unwrap().uci, "e2e4");
        assert_eq!(second_node.game_move.as_ref().unwrap().ply_number, 1);
        assert_eq!(
            second_node
                .game_move
                .as_ref()
                .unwrap()
                .position
                .as_ref()
                .unwrap()
                .fen,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );

        // traverse to the last node
        for _ in 0..tree.nodes.len() {
            tree.next_move(Some(0));
        }

        let last_node = &tree.nodes[tree.current_node_id.unwrap()];
        assert_eq!(last_node.game_move.as_ref().unwrap().san, "d5");
        assert_eq!(last_node.game_move.as_ref().unwrap().uci, "d7d5");
        assert_eq!(last_node.game_move.as_ref().unwrap().ply_number, 4);
    }

    #[test]
    fn parse_pgn_tokens_game_with_variations() {
        // Tests a game with a move with a variation
        // Per PGN spec: RAV represents "unplaying" the immediately prior move
        // 1. e4 (d4) e5 means:
        // - White plays e4, then Black plays e5 (main line)
        // - Variation: White plays d4 instead of e4 from starting position

        let tokens = vec![
            PgnToken::MoveNumber { number: 1 },
            PgnToken::Move {
                notation: "e4".to_string(),
            },
            PgnToken::Variation {
                moves: vec![PgnToken::Move {
                    notation: "d4".to_string(),
                }],
            },
            PgnToken::Move {
                notation: "e5".to_string(),
            },
        ];

        let tree =
            pgn_tokens_to_move_tree(1, ChessPosition::default(), &tokens)
                .unwrap();
        assert_eq!(
            tree.nodes.len(),
            4,
            "Tree should have 4 nodes (root + 3 moves)"
        );

        // Root node
        let first_node = &tree.nodes[tree.current_node_id.unwrap()];
        assert!(first_node.game_move.is_none());
        assert_eq!(first_node.children_ids.len(), 2); // Root has 2 children: e4 and d4 (variation)

        // First move (e4) - main line
        let second_node = &tree.nodes[first_node.children_ids[0]];
        assert_eq!(second_node.game_move.as_ref().unwrap().san, "e4");
        assert_eq!(second_node.game_move.as_ref().unwrap().uci, "e2e4");
        assert_eq!(second_node.game_move.as_ref().unwrap().ply_number, 1);
        assert_eq!(second_node.children_ids.len(), 1); // e4 has 1 child: e5

        // Variation move (d4) - alternative to e4
        let third_node = &tree.nodes[first_node.children_ids[1]];
        assert_eq!(third_node.game_move.as_ref().unwrap().san, "d4");
        assert_eq!(third_node.game_move.as_ref().unwrap().uci, "d2d4");
        assert_eq!(third_node.game_move.as_ref().unwrap().ply_number, 1);
        assert_eq!(third_node.children_ids.len(), 0); // d4 has no children in this test

        // Main line continuation (e5) - response to e4
        let fourth_node = &tree.nodes[second_node.children_ids[0]];
        assert_eq!(fourth_node.game_move.as_ref().unwrap().san, "e5");
        assert_eq!(fourth_node.game_move.as_ref().unwrap().uci, "e7e5");
        assert_eq!(fourth_node.game_move.as_ref().unwrap().ply_number, 2);
    }
}
