use std::fmt;

use crate::{
    database::{self, DatabaseError},
    models::{
        db::{Game, Header, Move},
        game::ParsingGame,
    },
};

#[derive(Debug)]
pub enum LoaderError {
    ParseError(String),
    DatabaseError(DatabaseError),
    ChessError(String),
    InvalidMove { move_text: String, reason: String },
    InvalidPosition { fen: Option<String>, reason: String },
    InvalidGameState { game_id: i32, reason: String },
    EmptyInput,
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoaderError::ParseError(e) => write!(f, "Parse error: {}", e),
            LoaderError::DatabaseError(e) => write!(f, "Database error: {}", e),
            LoaderError::ChessError(e) => write!(f, "Chess error: {}", e),
            LoaderError::InvalidMove { move_text, reason } => {
                write!(f, "Invalid move '{}': {}", move_text, reason)
            }
            LoaderError::InvalidPosition { fen, reason } => match fen {
                Some(fen) => write!(f, "Invalid position '{}': {}", fen, reason),
                None => write!(f, "Invalid position: {}", reason),
            },
            LoaderError::InvalidGameState { game_id, reason } => {
                write!(f, "Invalid game state for game {}: {}", game_id, reason)
            }
            LoaderError::EmptyInput => write!(f, "Empty PGN input"),
        }
    }
}

impl From<DatabaseError> for LoaderError {
    fn from(err: DatabaseError) -> Self {
        LoaderError::DatabaseError(err)
    }
}

/// A result from loading a PGN file
#[derive(Debug, Clone)]
pub struct LoadResult {
    pub games: Vec<ParsingGame>,
    pub success: bool,
    pub errors: Vec<String>,
}

impl LoadResult {
    pub fn new() -> Self {
        LoadResult {
            games: Vec::new(),
            success: true,
            errors: Vec::new(),
        }
    }

    pub fn get_games(&self) -> &Vec<ParsingGame> {
        &self.games
    }

    pub fn add_error(&mut self, error: String) {
        self.success = false;
        self.errors.push(error);
    }
}

/// Process a single PGN tag and update the game accordingly
fn process_tag(game: &mut ParsingGame, key: &str, value: &str) -> Result<(), LoaderError> {
    if key.trim().is_empty() {
        return Err(LoaderError::InvalidGameState {
            game_id: game.game.id.unwrap_or(-1),
            reason: "Empty tag key".to_string(),
        });
    }

    // Extract important tags that we want for all game metadata
    match key.trim() {
        "White" => {
            game.game.player_white = Some(value.to_string());
        }
        "Black" => {
            game.game.player_black = Some(value.to_string());
        }
        "Event" => {
            game.game.event = Some(value.to_string());
        }
        "Date" => {
            game.game.date_text = Some(value.to_string());
        }
        "Result" => {
            game.game.result = Some(value.to_string());
        }
        _ => {}
    }

    // Store all tags in headers vector, including the important ones
    game.headers.push(Header {
        game_id: game.game.id.unwrap_or(-1),
        header_key: key.trim().to_string(),
        header_value: value.to_string(),
        id: None,
    });

    Ok(())
}

/// Process a chess move and update the game state
fn process_move(
    game: &mut ParsingGame,
    mv: &str,
    move_number: i32,
    variation_id: i32,
) -> Result<(), LoaderError> {
    // Parse the move string into a shakmaty san object
    let san_obj =
        shakmaty::san::San::from_ascii(mv.as_bytes()).map_err(|e| LoaderError::InvalidMove {
            move_text: mv.to_string(),
            reason: e.to_string(),
        })?;

    let mut before_move_fen: Option<String> = None;
    let mut after_move_fen: Option<String> = None;

    // Process the move in the current game state
    let chess_game = game
        .chess_position
        .as_mut()
        .ok_or_else(|| LoaderError::InvalidGameState {
            game_id: game.game.id.unwrap_or(-1),
            reason: format!("No chess position available for move: {}", mv),
        })?;

    before_move_fen = Some(
        shakmaty::Position::board(chess_game)
            .board_fen(shakmaty::Position::promoted(chess_game))
            .to_string(),
    );

    // Convert and validate the move
    let mv_obj = san_obj
        .to_move(chess_game)
        .map_err(|e| LoaderError::InvalidMove {
            move_text: mv.to_string(),
            reason: e.to_string(),
        })?;

    if !shakmaty::Position::is_legal(chess_game, &mv_obj) {
        return Err(LoaderError::InvalidMove {
            move_text: mv.to_string(),
            reason: "Illegal move".to_string(),
        });
    }

    shakmaty::Position::play_unchecked(chess_game, &mv_obj);
    after_move_fen = Some(
        shakmaty::Position::board(chess_game)
            .board_fen(shakmaty::Position::promoted(chess_game))
            .to_string(),
    );

    // Handle position database operations
    let before_move_position_id = match before_move_fen.as_ref() {
        Some(fen) => match database::position::get_position_id_by_fen(fen)? {
            Some(id) => id,
            None => database::position::create_position(fen)?,
        },
        None => {
            return Err(LoaderError::InvalidPosition {
                fen: None,
                reason: "Failed to get position before move".to_string(),
            });
        }
    };

    let after_move_position_id = match after_move_fen.as_ref() {
        Some(fen) => match database::position::get_position_id_by_fen(fen)? {
            Some(id) => id,
            None => database::position::create_position(fen)?,
        },
        None => {
            return Err(LoaderError::InvalidPosition {
                fen: None,
                reason: "Failed to get position after move".to_string(),
            });
        }
    };

    // Create and store the move
    game.moves.push(Move {
        id: None,
        game_id: game.game.id.unwrap_or(-1),
        move_san: mv.to_string(),
        move_number,
        variation_order: Some(variation_id),
        parent_position_id: before_move_position_id,
        child_position_id: after_move_position_id,
        annotation: None,
    });

    Ok(())
}

/// Process a variation (sequence of alternative moves)
fn process_variation(
    game: &mut ParsingGame,
    tokens: &[crate::parser::PgnToken],
    move_number: &mut u32,
    variation_id: &mut u32,
) -> Result<(), LoaderError> {
    if tokens.is_empty() {
        return Err(LoaderError::InvalidGameState {
            game_id: game.game.id.unwrap(),
            reason: "Empty variation".to_string(),
        });
    }

    *variation_id += 1;
    let saved_move_number = *move_number;
    let saved_variation_id = *variation_id;

    // Create a scope to ensure state is restored even if an error occurs
    let result = (|| {
        for token in tokens {
            match token {
                crate::parser::PgnToken::Move(mv) => {
                    process_move(game, mv, *move_number as i32, *variation_id as i32)?;
                }
                crate::parser::PgnToken::MoveNumber(num) => {
                    *move_number = *num;
                }
                crate::parser::PgnToken::Variation(tokens) => {
                    process_variation(game, tokens, move_number, variation_id)?;
                }
                _ => {}
            }
        }
        Ok(())
    })();

    // Always restore the state
    *move_number = saved_move_number;
    *variation_id = saved_variation_id - 1;

    result
}

/// Create a new game with the given PGN
fn create_new_game(pgn: &str) -> Result<ParsingGame, LoaderError> {
    if pgn.trim().is_empty() {
        return Err(LoaderError::EmptyInput);
    }

    Ok(ParsingGame::from(Game {
        id: None, // Let the database assign the ID
        pgn: pgn.to_string(),
        ..Default::default()
    }))
}

/// Save game data to the database
fn save_game_data(result: &LoadResult) -> Result<(), LoaderError> {
    if result.games.is_empty() {
        return Ok(()); // No games to save
    }

    // Save all games to the database and get their assigned IDs
    let games: Vec<Game> = result.games.iter().map(|g| g.game.clone()).collect();
    let game_ids = database::game::insert_games_returning_ids(&games)?;

    // Now games have been assigned IDs by the database
    // Update the moves and headers with the correct game IDs
    for (game_index, game) in result.games.iter().enumerate() {
        if !game.moves.is_empty() {
            let mut moves = game.moves.clone();
            for move_entry in moves.iter_mut() {
                move_entry.game_id = game_ids[game_index];
            }
            database::move_::insert_moves(&moves)?;
        }
        if !game.headers.is_empty() {
            let mut headers = game.headers.clone();
            for header in headers.iter_mut() {
                header.game_id = game_ids[game_index];
            }
            database::header::insert_headers(&headers)?;
        }
    }

    Ok(())
}

pub fn load_pgn(pgn: &str) -> Result<LoadResult, LoaderError> {
    let mut result = LoadResult::new();

    if pgn.trim().is_empty() {
        return Err(LoaderError::EmptyInput);
    }

    // Parse the pgn (returns a vector of tokens or errors)
    let tokens = crate::parser::parse_pgn(pgn)
        .map_err(|e| LoaderError::ParseError(format!("Failed to parse PGN: {:?}", e)))?;

    if tokens.is_empty() {
        return Err(LoaderError::ParseError(
            "No tokens found in PGN".to_string(),
        ));
    }

    // Create a new game result for the first game
    let mut current_game = create_new_game(pgn)?;

    let mut current_game_move_number: u32 = 0;
    let mut current_game_variation_id: u32 = 0;
    let mut game_started = false;

    for token in tokens.iter() {
        // If we've already seen a game's moves, we should consider this a new game
        if game_started && matches!(token, crate::parser::PgnToken::Tag(_, _)) {
            if !current_game.moves.is_empty() {
                result.games.push(current_game);
                current_game = create_new_game(pgn)?;
                current_game_move_number = 0;
                current_game_variation_id = 0;
                game_started = false;
            }
        }

        match token {
            crate::parser::PgnToken::Tag(key, value) => {
                if let Err(e) = process_tag(&mut current_game, key, value) {
                    result.add_error(e.to_string());
                    continue;
                }
            }
            crate::parser::PgnToken::Move(mv) => {
                game_started = true;
                if let Err(e) = process_move(
                    &mut current_game,
                    mv,
                    current_game_move_number as i32,
                    current_game_variation_id as i32,
                ) {
                    result.add_error(e.to_string());
                    continue;
                }
            }
            crate::parser::PgnToken::MoveNumber(num) => {
                current_game_move_number = *num;
            }
            crate::parser::PgnToken::Variation(tokens) => {
                if let Err(e) = process_variation(
                    &mut current_game,
                    tokens,
                    &mut current_game_move_number,
                    &mut current_game_variation_id,
                ) {
                    result.add_error(e.to_string());
                    continue;
                }
            }
            crate::parser::PgnToken::Comment(comment) => {
                if let Some(last_move) = current_game.moves.last_mut() {
                    last_move.annotation = Some(comment.to_string());
                }
            }
            crate::parser::PgnToken::Result(_) => {
                // Store game result if needed
            }
            _ => {
                result.add_error(format!("Unexpected token: {:?}", token));
            }
        }
    }

    // Don't forget to add the last game
    if !current_game.moves.is_empty() {
        result.games.push(current_game);
    }

    save_game_data(&result)?;

    if result.games.is_empty() {
        return Err(LoaderError::ParseError(
            "No valid games found in PGN".to_string(),
        ));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to run tests with a database connection
    fn with_test_db<F>(test: F) -> Result<(), DatabaseError>
    where
        F: FnOnce() -> Result<(), DatabaseError>,
    {
        // Run the test (database is initialized with migrations in establish_connection)
        test()
    }

    #[test]
    fn test_empty_pgn() {
        with_test_db(|| {
            // Test completely empty string
            let result = load_pgn("");
            assert!(matches!(result, Err(LoaderError::EmptyInput)));

            // Test whitespace-only string
            let result = load_pgn("   \n  \t  ");
            assert!(matches!(result, Err(LoaderError::EmptyInput)));

            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_basic_game() {
        with_test_db(|| {
            let pgn = r#"[Event "Test Game"]
[Site "Chess Club"]
[Date "2024.03.21"]
[Round "1"]
[White "Player 1"]
[Black "Player 2"]
[Result "1-0"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 1-0"#;

            let result = load_pgn(pgn).unwrap();

            assert!(result.success);
            assert_eq!(result.errors.len(), 0);
            assert_eq!(result.games.len(), 1);

            let game = &result.games[0];

            assert_eq!(game.game.player_white, Some("Player 1".to_string()));
            assert_eq!(game.game.player_black, Some("Player 2".to_string()));
            assert_eq!(game.game.event, Some("Test Game".to_string()));
            assert_eq!(game.game.result, Some("1-0".to_string()));
            assert_eq!(game.moves.len(), 5);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_multiple_games() {
        with_test_db(|| {
            let pgn = r#"[Event "Game 1"]
[White "Player 1"]
[Black "Player 2"]
[Result "1-0"]

1. e4 e5 2. Nf3 1-0

[Event "Game 2"]
[White "Player 3"]
[Black "Player 4"]
[Result "0-1"]

1. d4 d5 0-1"#;

            let result = load_pgn(pgn).unwrap();
            assert!(result.success);
            assert_eq!(result.games.len(), 2);

            assert_eq!(result.games[0].game.event, Some("Game 1".to_string()));
            assert_eq!(result.games[1].game.event, Some("Game 2".to_string()));
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_custom_headers() {
        with_test_db(|| {
            let pgn = r#"[Event "Test Game"]
[White "Player 1"]
[Black "Player 2"]
[Result "1-0"]
[CustomHeader1 "Value1"]
[CustomHeader2 "Value2"]

1. e4 e5 1-0"#;

            let result = load_pgn(pgn).unwrap();
            let game = &result.games[0];

            let custom_headers: Vec<_> = game
                .headers
                .iter()
                .filter(|h| h.header_key == "CustomHeader1" || h.header_key == "CustomHeader2")
                .collect();

            assert_eq!(custom_headers.len(), 2);
            assert!(custom_headers
                .iter()
                .any(|h| h.header_key == "CustomHeader1" && h.header_value == "Value1"));
            assert!(custom_headers
                .iter()
                .any(|h| h.header_key == "CustomHeader2" && h.header_value == "Value2"));
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_invalid_move() {
        with_test_db(|| {
            let pgn = r#"[Event "Test Game"]
[White "Player 1"]
[Black "Player 2"]
[Result "*"]

1. e4 e5 2. InvalidMove e6 3. Nf3 Nc6 *"#;

            let result = load_pgn(pgn).unwrap();
            assert!(!result.success);

            // Check that we have the error message for the invalid move
            let error_message = result
                .errors
                .iter()
                .find(|e| e.contains("InvalidMove"))
                .expect("Should contain error about invalid move");
            assert!(error_message.contains("Invalid move"));

            // Verify that valid moves before and after the invalid one were processed
            let game = &result.games[0];
            assert_eq!(game.moves.len(), 4); // e4, e5, Nf3, Nc6

            // Verify the moves are the correct ones
            assert!(game.moves.iter().any(|m| m.move_san == "e4"));
            assert!(game.moves.iter().any(|m| m.move_san == "e5"));
            assert!(game.moves.iter().any(|m| m.move_san == "Nf3"));
            assert!(game.moves.iter().any(|m| m.move_san == "Nc6"));

            // Verify move numbers are correct
            let first_moves: Vec<_> = game.moves.iter().filter(|m| m.move_number == 1).collect();
            assert_eq!(first_moves.len(), 2); // e4, e5

            let third_moves: Vec<_> = game.moves.iter().filter(|m| m.move_number == 3).collect();
            assert_eq!(third_moves.len(), 2); // Nf3, Nc6

            Ok(())
        })
        .unwrap();
    }

    #[test]
    // Skip this test for now, need to revisit variations
    #[ignore]
    fn test_game_with_variations() {
        with_test_db(|| {
            let pgn = r#"[Event "Test Game"]
[White "Player 1"]
[Black "Player 2"]
[Result "1-0"]

1. e4 e5 2. Nf3 (2. d4 d5 3. exd5) 2... Nc6 3. Bb5 1-0"#;

            let result = load_pgn(pgn).unwrap();
            assert!(result.success);

            let game = &result.games[0];
            // Verify main line moves
            assert!(game.moves.iter().any(|m| m.move_san == "e4"));
            assert!(game.moves.iter().any(|m| m.move_san == "Nf3"));

            // Verify variation moves have different variation_order
            let variation_moves: Vec<_> = game
                .moves
                .iter()
                .filter(|m| m.variation_order.is_some() && m.variation_order.unwrap() > 0)
                .collect();
            assert!(!variation_moves.is_empty());
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_game_with_comments() {
        with_test_db(|| {
            let pgn = r#"[Event "Test Game"]
[White "Player 1"]
[Black "Player 2"]
[Result "1-0"]

1. e4 {Good opening move} e5 2. Nf3 {Developing knight} Nc6 1-0"#;

            let result = load_pgn(pgn).unwrap();
            assert!(result.success);

            let game = &result.games[0];
            let moves_with_comments: Vec<_> = game
                .moves
                .iter()
                .filter(|m| m.annotation.is_some())
                .collect();

            assert_eq!(moves_with_comments.len(), 2);
            assert!(moves_with_comments
                .iter()
                .any(|m| m.annotation.as_ref().unwrap() == "Good opening move"));
            assert!(moves_with_comments
                .iter()
                .any(|m| m.annotation.as_ref().unwrap() == "Developing knight"));
            Ok(())
        })
        .unwrap();
    }
}
