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
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoaderError::ParseError(e) => write!(f, "Parse error: {}", e),
            LoaderError::DatabaseError(e) => write!(f, "Database error: {}", e),
            LoaderError::ChessError(e) => write!(f, "Chess error: {}", e),
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

/// Loads and parses a PGN (Portable Game Notation) string into a structured format
///
/// # Arguments
/// * `pgn` - A string slice containing the PGN data to parse
///
/// # Returns
/// * `Result<LoadResult, LoaderError>` - A Result containing either:
///   - `LoadResult`: Contains the parsed games, success status, and any non-fatal errors
///   - `LoaderError`: Contains fatal errors that prevented parsing
///
/// # Error Handling
/// The function will return a LoaderError in the following cases:
/// - Invalid PGN syntax
/// - Database operations fail
/// - Invalid chess moves
///
/// Non-fatal errors (like invalid moves in variations) are stored in the game's errors vector
pub fn load_pgn(pgn: &str) -> Result<LoadResult, LoaderError> {
    let mut result = LoadResult::new();

    if pgn.trim().is_empty() {
        return Err(LoaderError::ParseError("Empty PGN string".to_string()));
    }

    // Parse the pgn (returns a vector of tokens or errors)
    let tokens = crate::parser::parse_pgn(pgn)
        .map_err(|e| LoaderError::ParseError(format!("Failed to parse PGN: {:?}", e)))?;

    let mut game_started = false;
    let game_count = database::game::get_game_id_count()?;

    // Create a new game result for the first game
    let mut current_game = ParsingGame::from(Game {
        id: Some(((game_count as i64) + 1) as i32),
        pgn: pgn.to_string(),
        ..Default::default()
    });

    let mut current_game_move_number: u32 = 0;
    let mut current_game_variation_id: u32 = 0;

    for token in tokens.iter() {
        // If we've already seen a game's moves, we should consider this a new game
        if game_started && matches!(token, crate::parser::PgnToken::Tag(_, _)) {
            if !current_game.moves.is_empty() {
                result.games.push(current_game);
                current_game = ParsingGame::from(Game {
                    id: Some(((game_count as i64 + result.games.len() as i64) + 1) as i32),
                    pgn: pgn.to_string(),
                    ..Default::default()
                });
                current_game_move_number = 0;
                current_game_variation_id = 0;
                game_started = false;
            }
        }

        match token {
            crate::parser::PgnToken::Tag(key, value) => {
                // Check for archival tags that we track for all games
                match key.trim() {
                    "White" => {
                        current_game.game.player_white = Some(value.clone());
                    }
                    "Black" => {
                        current_game.game.player_black = Some(value.clone());
                    }
                    "Event" => {
                        current_game.game.event = Some(value.clone());
                    }
                    "Date" => {
                        current_game.game.date_text = Some(value.clone());
                    }
                    "Result" => {
                        current_game.game.result = Some(value.clone());
                    }
                    _ => {
                        // Store all other tags outside the 7-archive tags
                        // in the headers vector
                        current_game.headers.push(Header {
                            game_id: current_game.game.id.unwrap(),
                            header_key: key.trim().to_string(),
                            header_value: value.clone(),
                            id: None,
                        });
                    }
                }
            }
            crate::parser::PgnToken::Move(mv) => {
                game_started = true;

                // Parse the move string into a shakmaty san object
                let san_obj = match shakmaty::san::San::from_ascii(mv.as_bytes()) {
                    Ok(san) => san,
                    Err(e) => {
                        current_game
                            .errors
                            .push(format!("Invalid SAN move {}: {}", mv, e));
                        result.add_error(format!("Invalid SAN move {}: {}", mv, e));
                        continue;
                    }
                };

                let mut before_move_fen: Option<String> = None;
                let mut after_move_fen: Option<String> = None;

                // Process the move in the current game state
                if let Some(ref mut game) = current_game.chess_position {
                    before_move_fen = Some(
                        shakmaty::Position::board(game)
                            .board_fen(shakmaty::Position::promoted(game))
                            .to_string(),
                    );

                    // Convert and validate the move
                    let mv_obj = match san_obj.to_move(game) {
                        Ok(mv) => mv,
                        Err(e) => {
                            current_game
                                .errors
                                .push(format!("Invalid move {}: {}", mv, e));
                            result.add_error(format!("Invalid move {}: {}", mv, e));
                            continue;
                        }
                    };

                    if !shakmaty::Position::is_legal(game, &mv_obj) {
                        current_game.errors.push(format!("Illegal move: {}", mv));
                        result.add_error(format!("Illegal move: {}", mv));
                        continue;
                    }

                    shakmaty::Position::play_unchecked(game, &mv_obj);
                    after_move_fen = Some(
                        shakmaty::Position::board(game)
                            .board_fen(shakmaty::Position::promoted(game))
                            .to_string(),
                    );
                } else {
                    current_game
                        .errors
                        .push(format!("No chess position available for move: {}", mv));
                    continue;
                }

                // Handle position database operations
                let before_move_position_id = match before_move_fen.as_ref() {
                    Some(fen) => match database::position::get_position_id_by_fen(fen)? {
                        Some(id) => id,
                        None => database::position::create_position(fen)?,
                    },
                    None => {
                        current_game
                            .errors
                            .push("Failed to get position before move".to_string());
                        continue;
                    }
                };

                let after_move_position_id = match after_move_fen.as_ref() {
                    Some(fen) => match database::position::get_position_id_by_fen(fen)? {
                        Some(id) => id,
                        None => database::position::create_position(fen)?,
                    },
                    None => {
                        current_game
                            .errors
                            .push("Failed to get position after move".to_string());
                        continue;
                    }
                };

                // Create and store the move
                let move_object = Move {
                    game_id: current_game.game.id.unwrap(),
                    move_san: mv.to_string(),
                    move_number: current_game_move_number as i32,
                    variation_order: Some(current_game_variation_id as i32),
                    parent_position_id: before_move_position_id,
                    child_position_id: after_move_position_id,
                    ..Default::default()
                };
                current_game.moves.push(move_object);
            }
            crate::parser::PgnToken::MoveNumber(num) => {
                current_game_move_number = *num;
            }
            crate::parser::PgnToken::Variation(tokens) => {
                current_game_variation_id += 1;
                // Store the current state
                let saved_move_number = current_game_move_number;
                let saved_variation_id = current_game_variation_id;

                // Process variation moves
                for token in tokens {
                    match token {
                        crate::parser::PgnToken::Move(mv) => {
                            // Handle moves in variation similar to main line
                            // but with the current variation ID
                            if let Some(ref mut game) = current_game.chess_position {
                                if let Ok(san_obj) = shakmaty::san::San::from_ascii(mv.as_bytes()) {
                                    if let Ok(mv_obj) = san_obj.to_move(game) {
                                        if shakmaty::Position::is_legal(game, &mv_obj) {
                                            // Add move to the variation
                                            current_game.moves.push(Move {
                                                game_id: current_game.game.id.unwrap(),
                                                move_san: mv.to_string(),
                                                move_number: current_game_move_number as i32,
                                                variation_order: Some(
                                                    current_game_variation_id as i32,
                                                ),
                                                ..Default::default()
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        crate::parser::PgnToken::MoveNumber(num) => {
                            current_game_move_number = *num;
                        }
                        _ => {}
                    }
                }

                // Restore the state after processing variation
                current_game_move_number = saved_move_number;
                current_game_variation_id = saved_variation_id - 1;
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
                current_game
                    .errors
                    .push(format!("Unexpected token: {:?}", token));
            }
        }
    }

    // Don't forget to add the last game
    if !current_game.moves.is_empty() {
        result.games.push(current_game);
    }

    // Save all games to the database
    let games: Vec<Game> = result.games.iter().map(|g| g.game.clone()).collect();
    database::game::insert_games(&games)?;

    // Save all moves
    for game in &result.games {
        database::move_::insert_moves(&game.moves)?;
    }

    // Save all headers
    for game in &result.games {
        database::header::insert_headers(&game.headers)?;
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
            let result = load_pgn("");
            assert!(matches!(result, Err(LoaderError::ParseError(_))));
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

1. e4 e5 2. InvalidMove e6 *"#;

            let result = load_pgn(pgn).unwrap();
            assert!(!result.success);
            assert!(result.errors.len() > 0);
            assert_eq!(result.games[0].moves.len(), 2); // Only valid moves should be stored
            Ok(())
        })
        .unwrap();
    }

    #[test]
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
