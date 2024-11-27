use std::fmt;

use crate::{
    database::{self, DatabaseError},
    models::{
        db::{Game, Move},
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

pub fn load_pgn(pgn: &str) -> Result<LoadResult, LoaderError> {
    let mut result = LoadResult::new();

    // Parse the pgn (returns a vector of tokens or errors)
    let tokens = crate::parser::parse_pgn(pgn)
        .map_err(|e| LoaderError::ParseError(format!("Failed to parse PGN: {:?}", e)))?;

    let mut game_started = false;
    let game_count = database::game::get_game_id_count()?;

    // Create a new game result for the first game
    let mut current_game = ParsingGame::from(Game {
        id: Some(((game_count as i64) + 1) as i32),
        ..Default::default()
    });

    let mut current_game_move_number: u32 = 0;
    let mut current_game_variation_id: u32 = 0;

    for token in tokens.iter() {
        // If we've already seen a game's moves, we should consider this a new game
        if game_started && matches!(token, crate::parser::PgnToken::Tag(_, _)) {
            result.games.push(current_game);
            current_game = ParsingGame::from(Game {
                id: Some(((game_count as i64 + result.games.len() as i64) + 1) as i32),
                ..Default::default()
            });
            current_game_move_number = 0;
            current_game_variation_id = 0;
            game_started = false;
        }

        match token {
            crate::parser::PgnToken::Tag(key, value) => {
                match key.as_str() {
                    "White" => current_game.game.player_white = Some(value.clone()),
                    "Black" => current_game.game.player_black = Some(value.clone()),
                    "Event" => current_game.game.event = Some(value.clone()),
                    "Date" => current_game.game.date_text = Some(value.clone()),
                    "Result" => current_game.game.result = Some(value.clone()),
                    _ => {} // Ignore other tags for now
                }
            }
            crate::parser::PgnToken::Move(mv) => {
                game_started = true;

                // Parse the move string into a shakmaty san object
                let san_obj = shakmaty::san::San::from_ascii(mv.as_bytes()).map_err(|e| {
                    LoaderError::ChessError(format!("Invalid SAN move {}: {}", mv, e))
                })?;

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
                    let mv_obj = san_obj.to_move(game).map_err(|e| {
                        LoaderError::ChessError(format!("Invalid move {}: {}", mv, e))
                    })?;

                    if !shakmaty::Position::is_legal(game, &mv_obj) {
                        current_game.errors.push(format!("Invalid move: {}", mv));
                        continue;
                    }

                    shakmaty::Position::play_unchecked(game, &mv_obj);
                    after_move_fen = Some(
                        shakmaty::Position::board(game)
                            .board_fen(shakmaty::Position::promoted(game))
                            .to_string(),
                    );
                }

                // Handle position database operations
                let before_move_position_id = match database::position::get_position_id_by_fen(
                    before_move_fen.as_ref().unwrap(),
                )? {
                    Some(id) => id,
                    None => database::position::create_position(before_move_fen.as_ref().unwrap())?,
                };

                let after_move_position_id = match database::position::get_position_id_by_fen(
                    after_move_fen.as_ref().unwrap(),
                )? {
                    Some(id) => id,
                    None => database::position::create_position(after_move_fen.as_ref().unwrap())?,
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
                // TODO: recursively parse the variation
                current_game_variation_id -= 1;
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

    Ok(result)
}
