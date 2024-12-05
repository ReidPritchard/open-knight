use std::fmt;

use crate::{
    database::{self, Database, DatabaseError, DbConnection},
    models::{
        db::{Game, Header, Move},
        game::ParsingGame,
    },
    state,
};

#[derive(Debug)]
pub enum LoaderError {
    ParseError(String),
    DatabaseError(database::DatabaseError),
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

impl From<database::DatabaseError> for LoaderError {
    fn from(err: database::DatabaseError) -> Self {
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

pub fn load_pgn(pgn: &str, db: &Database) -> Result<LoadResult, LoaderError> {
    println!("=== load_pgn called ===");

    if pgn.trim().is_empty() {
        println!("Empty PGN input");
        return Err(LoaderError::EmptyInput);
    }

    // Parse the pgn (returns a vector of tokens or errors)
    println!("Parsing PGN input");
    let tokens = crate::parser::parse_pgn(pgn)
        .map_err(|e| LoaderError::ParseError(format!("Failed to parse PGN: {:?}", e)))?;

    println!("Number of tokens parsed: {}", tokens.len());

    if tokens.is_empty() {
        println!("No tokens found in PGN");
        return Err(LoaderError::ParseError(
            "No tokens found in PGN".to_string(),
        ));
    }

    // Process everything in a single transaction
    db.transaction(|conn| {
        let mut result = LoadResult::new();

        // Create a new game result for the first game
        let mut current_game =
            create_new_game(pgn).map_err(|e| DatabaseError::TransactionError(e.to_string()))?;
        println!("Created first game object");

        let mut current_game_move_number: u32 = 0;
        let mut current_game_variation_id: u32 = 0;
        let mut game_started = false;

        println!("Starting token processing loop");
        for (i, token) in tokens.iter().enumerate() {
            println!("Processing token {} of {}", i + 1, tokens.len());

            // If we've already seen a game's moves, we should consider this a new game
            if game_started && matches!(token, crate::parser::PgnToken::Tag(_, _)) {
                if !current_game.moves.is_empty() {
                    println!(
                        "Found new game marker, current game has {} moves",
                        current_game.moves.len()
                    );
                    // If the last game has moves (assume it's a complete game), add it to the result
                    result.games.push(current_game);
                    println!(
                        "Added game to results, total games now: {}",
                        result.games.len()
                    );
                    current_game = create_new_game(pgn)
                        .map_err(|e| DatabaseError::TransactionError(e.to_string()))?;
                    current_game_move_number = 0;
                    current_game_variation_id = 0;
                    game_started = false;
                }
            }

            match token {
                crate::parser::PgnToken::Tag(key, value) => {
                    println!("Processing tag: {} = {}", key, value);
                    if let Err(e) = process_tag(&mut current_game, key, value) {
                        println!("Error processing tag: {}", e);
                        result.add_error(e.to_string());
                        continue;
                    }
                }
                crate::parser::PgnToken::Move(mv) => {
                    println!("Processing move: {}", mv);
                    game_started = true;
                    if let Err(e) = process_move_with_conn(
                        conn,
                        &mut current_game,
                        mv,
                        current_game_move_number as i32,
                        current_game_variation_id as i32,
                    ) {
                        println!("Error processing move: {}", e);
                        result.add_error(e.to_string());
                        continue;
                    }
                }
                crate::parser::PgnToken::MoveNumber(num) => {
                    println!("Processing move number: {}", num);
                    current_game_move_number = *num;
                }
                crate::parser::PgnToken::Variation(tokens) => {
                    println!("Processing variation with {} tokens", tokens.len());
                    if let Err(e) = process_variation_with_conn(
                        conn,
                        &mut current_game,
                        tokens,
                        &mut current_game_move_number,
                        &mut current_game_variation_id,
                    ) {
                        println!("Error processing variation: {}", e);
                        result.add_error(e.to_string());
                        continue;
                    }
                }
                crate::parser::PgnToken::Comment(comment) => {
                    println!("Processing comment: {}", comment);
                    if let Some(last_move) = current_game.moves.last_mut() {
                        last_move.annotation = Some(comment.to_string());
                    }
                }
                crate::parser::PgnToken::Result(_) => {
                    println!("Processing game result token");
                }
                _ => {
                    println!("Unexpected token: {:?}", token);
                    result.add_error(format!("Unexpected token: {:?}", token));
                }
            }
        }

        // Don't forget to add the last game
        if !current_game.moves.is_empty() {
            println!("Adding final game with {} moves", current_game.moves.len());
            result.games.push(current_game);
        }

        println!("Saving game data");
        save_game_data_with_conn(conn, &result)
            .map_err(|e| DatabaseError::TransactionError(e.to_string()))?;

        println!("Final number of games: {}", result.games.len());
        if result.games.is_empty() {
            println!("No valid games found in PGN");
            return Err(DatabaseError::TransactionError(
                "No valid games found in PGN".to_string(),
            ));
        }

        Ok(result)
    })
    .map_err(|e| match e {
        database::DatabaseError::QueryError(e) => {
            LoaderError::DatabaseError(database::DatabaseError::QueryError(e))
        }
        e => LoaderError::DatabaseError(e),
    })
}

fn process_move_with_conn(
    conn: &mut DbConnection,
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
        Some(fen) => {
            let existing_id = database::position::get_position_id_by_fen_with_conn(conn, fen)?;
            match existing_id {
                Some(id) => id,
                None => database::position::create_position_with_conn(conn, fen)?,
            }
        }
        None => {
            return Err(LoaderError::InvalidPosition {
                fen: None,
                reason: "Failed to get position before move".to_string(),
            })
        }
    };

    let after_move_position_id = match after_move_fen.as_ref() {
        Some(fen) => {
            let existing_id = database::position::get_position_id_by_fen_with_conn(conn, fen)?;
            match existing_id {
                Some(id) => id,
                None => database::position::create_position_with_conn(conn, fen)?,
            }
        }
        None => {
            return Err(LoaderError::InvalidPosition {
                fen: None,
                reason: "Failed to get position after move".to_string(),
            })
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

fn process_variation_with_conn(
    conn: &mut DbConnection,
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
                    process_move_with_conn(
                        conn,
                        game,
                        mv,
                        *move_number as i32,
                        *variation_id as i32,
                    )?;
                }
                crate::parser::PgnToken::MoveNumber(num) => {
                    *move_number = *num;
                }
                crate::parser::PgnToken::Variation(tokens) => {
                    process_variation_with_conn(conn, game, tokens, move_number, variation_id)?;
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

fn save_game_data_with_conn(
    conn: &mut DbConnection,
    result: &LoadResult,
) -> Result<(), LoaderError> {
    println!("=== save_game_data_with_conn called ===");
    println!("Number of games to save: {}", result.games.len());

    if result.games.is_empty() {
        println!("No games to save, returning early");
        return Ok(()); // No games to save
    }

    // Save all games to the database and get their assigned IDs
    let games: Vec<Game> = result.games.iter().map(|g| g.game.clone()).collect();
    println!("Saving {} games to database", games.len());
    let game_ids = database::game::insert_games_returning_ids_with_conn(conn, &games)?;
    println!("Received {} game IDs from database", game_ids.len());

    // Now games have been assigned IDs by the database
    // Update the moves and headers with the correct game IDs
    for (game_index, game) in result.games.iter().enumerate() {
        println!(
            "Processing game {} of {}",
            game_index + 1,
            result.games.len()
        );
        println!(
            "Game has {} moves and {} headers",
            game.moves.len(),
            game.headers.len()
        );

        if !game.moves.is_empty() {
            let mut moves = game.moves.clone();
            for move_entry in moves.iter_mut() {
                move_entry.game_id = game_ids[game_index];
            }
            println!(
                "Inserting {} moves for game {}",
                moves.len(),
                game_ids[game_index]
            );
            database::move_::insert_moves_with_conn(conn, &moves)?;
        }
        if !game.headers.is_empty() {
            let mut headers = game.headers.clone();
            for header in headers.iter_mut() {
                header.game_id = game_ids[game_index];
            }
            println!(
                "Inserting {} headers for game {}",
                headers.len(),
                game_ids[game_index]
            );
            database::header::insert_headers_with_conn(conn, &headers)?;
        }
    }

    println!("=== save_game_data_with_conn completed successfully ===");
    Ok(())
}
