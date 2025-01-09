use std::fmt;

use crate::{
    database::{self, Database, DatabaseError},
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

pub async fn load_pgn(pgn: &str, db: &Database) -> Result<LoadResult, LoaderError> {
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

    let mut result = LoadResult::new();

    // Create a new game result for the first game
    let mut current_game = create_new_game(pgn)?;
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
                current_game = create_new_game(pgn)?;
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
                if let Err(e) = process_move(
                    db,
                    &mut current_game,
                    mv,
                    current_game_move_number as i32,
                    current_game_variation_id as i32,
                )
                .await
                {
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
                if let Err(e) = process_variation(
                    db,
                    &mut current_game,
                    tokens,
                    &mut current_game_move_number,
                    &mut current_game_variation_id,
                )
                .await
                {
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

    // Save all games to the database
    save_game_data(db, &result).await?;

    Ok(result)
}

async fn process_move(
    db: &Database,
    game: &mut ParsingGame,
    mv: &str,
    move_number: i32,
    variation_id: i32,
) -> Result<(), LoaderError> {
    // Create a new move
    let mut new_move = Move {
        id: None,
        game_id: game.game.id.unwrap_or(-1),
        move_number,
        move_san: mv.to_string(),
        annotation: None,
        variation_order: Some(variation_id),
        parent_position_id: 0, // Will be set later
        child_position_id: 0,  // Will be set later
    };

    // Get or create the parent position
    let parent_fen = if game.moves.is_empty() {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    } else if let Some(last_move) = game.moves.last() {
        // TODO: Get the FEN from the last move's child position
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
    } else {
        return Err(LoaderError::InvalidGameState {
            game_id: game.game.id.unwrap_or(-1),
            reason: "No previous move found".to_string(),
        });
    };

    let parent_position_id =
        match database::position::get_position_id_by_fen(db, &parent_fen).await? {
            Some(id) => id,
            None => database::position::create_position(db, &parent_fen).await?,
        };
    new_move.parent_position_id = parent_position_id;

    // TODO: Calculate the child position FEN
    let child_fen = parent_fen.clone(); // This is wrong, but we'll fix it later
    let child_position_id = match database::position::get_position_id_by_fen(db, &child_fen).await?
    {
        Some(id) => id,
        None => database::position::create_position(db, &child_fen).await?,
    };
    new_move.child_position_id = child_position_id;

    game.moves.push(new_move);
    Ok(())
}

async fn process_variation(
    db: &Database,
    game: &mut ParsingGame,
    tokens: &[crate::parser::PgnToken],
    move_number: &mut u32,
    variation_id: &mut u32,
) -> Result<(), LoaderError> {
    *variation_id += 1;
    let current_variation_id = *variation_id;

    for token in tokens {
        match token {
            crate::parser::PgnToken::Move(mv) => {
                process_move(
                    db,
                    game,
                    mv,
                    *move_number as i32,
                    current_variation_id as i32,
                )
                .await?;
            }
            crate::parser::PgnToken::MoveNumber(num) => {
                *move_number = *num;
            }
            crate::parser::PgnToken::Comment(comment) => {
                if let Some(last_move) = game.moves.last_mut() {
                    last_move.annotation = Some(comment.to_string());
                }
            }
            _ => {
                return Err(LoaderError::ParseError(format!(
                    "Unexpected token in variation: {:?}",
                    token
                )));
            }
        }
    }

    Ok(())
}

async fn save_game_data(db: &Database, result: &LoadResult) -> Result<(), LoaderError> {
    for game in &result.games {
        // Insert the game first to get its ID
        let game_ids = database::game::insert_games_returning_ids(db, &[game.game.clone()]).await?;
        let game_id = game_ids[0];

        // Update all moves and headers with the correct game ID
        let mut moves = game.moves.clone();
        let mut headers = game.headers.clone();
        for mv in &mut moves {
            mv.game_id = game_id;
        }
        for header in &mut headers {
            header.game_id = game_id;
        }

        // Insert moves and headers
        database::move_::insert_moves(db, &moves).await?;
        database::header::insert_headers(db, &headers).await?;
    }

    Ok(())
}
