use serde::Serialize;
use shakmaty::{Chess, Position};

use crate::{database, models::Move, parser};

/// A result from loading a PGN file
///
/// Can contain multiple games, moves, and headers
#[derive(Debug, Clone)]
pub struct LoadResult {
    pub games: Vec<GameResult>,
    pub success: bool,
}

/// Simple type for a single game+headers+errors result from the pgn loader
#[derive(Debug, Clone, Serialize)]
pub struct GameResult {
    pub id: i32,
    pub headers: Vec<(String, String)>,
    #[serde(skip)]
    pub game: Option<Chess>,
    pub moves: Vec<Move>,
    pub pgn: String,
    pub errors: Vec<String>,
}

impl GameResult {
    pub fn new() -> Self {
        GameResult {
            id: 0,
            headers: Vec::new(),
            game: Some(Chess::default()),
            moves: Vec::new(),
            pgn: String::new(),
            errors: Vec::new(),
        }
    }

    pub fn new_with_id(id: i32) -> Self {
        GameResult {
            id,
            ..GameResult::new()
        }
    }
}

impl LoadResult {
    pub fn get_game_results(&self) -> &Vec<GameResult> {
        &self.games
    }
}

pub fn load_pgn(pgn: &str) -> LoadResult {
    // Parse the pgn (returns a vector of tokens or errors)
    let tokens = parser::parse_pgn(pgn);

    let mut games: Vec<GameResult> = Vec::new();
    // Tokens can contain multiple games, so we need to loop through them
    // and determine where each game starts and ends while parsing
    if let Ok(tokens) = tokens {
        let mut game_started = false;

        // Create a new game result for the first game
        games.push(GameResult::new_with_id(
            ((database::get_game_id_count() as i64 + games.len() as i64) + 1) as i32,
        ));
        let mut current_game_idx = 0;
        // State for the current game
        let mut current_game_move_number: u32 = 0;
        let mut current_game_variation_id: u32 = 0;

        // State for all moves in all parsed games
        let mut total_move_count = 0;
        tokens.iter().for_each(|token| {
            // If we've already seen a game's moves, we should consider this a new game
            if game_started && matches!(token, parser::PgnToken::Tag(_, _)) {
                println!("Game complete, adding new game");
                games.push(GameResult::new_with_id(
                    ((database::get_game_id_count() as i64 + games.len() as i64) + 1) as i32,
                ));
                current_game_idx += 1;
                current_game_move_number = 0;
                current_game_variation_id = 0;

                game_started = false;
            }

            let game_result = games.get_mut(current_game_idx).unwrap();
            match token {
                parser::PgnToken::Tag(key, value) => {
                    println!("TAG: {} = {}", key, value);
                    game_result
                        .headers
                        .push((key.to_string(), value.to_string()));
                }
                parser::PgnToken::Move(mv) => {
                    game_started = true;
                    total_move_count += 1;

                    println!("MOVE: {}", mv);

                    // Parse the move string into a shakmaty san object (this removes the check/checkmate suffix)
                    let san_obj = shakmaty::san::San::from_ascii(mv.as_bytes()).unwrap();
                    let mut fen: Option<String> = None;

                    // See if we can play the move in the current game state
                    if let Some(ref mut game) = game_result.game {
                        // Get the FEN of the current position
                        fen = Some(game.board().board_fen(game.promoted()).to_string());

                        // Convert the move string into a move object and try to play it
                        let mv_obj = san_obj.to_move(game).unwrap();

                        let is_valid_move = game.is_legal(&mv_obj);
                        if !is_valid_move {
                            game_result.errors.push(format!("Invalid move: {}", mv));
                        } else {
                            // TODO: We probably need to handle variations here in order to avoid overwriting the game state
                            game.play_unchecked(&mv_obj);
                        }
                    }

                    // Convert the move token string into a move object
                    let move_object = Move {
                        // Generate a new id for the move (move db count + number of moves in games loaded so far)
                        id: (database::get_move_id_count() + total_move_count + 1) as i32,
                        game_id: game_result.id,
                        move_san: mv.to_string(),
                        move_number: current_game_move_number as i32,
                        variation_id: Some(current_game_variation_id as i32),
                        fen,
                        ..Default::default()
                    };
                    game_result.moves.push(move_object);
                }
                parser::PgnToken::MoveNumber(num) => {
                    println!("MOVE NUMBER: {}", num);
                    current_game_move_number = *num;
                }
                parser::PgnToken::Variation(tokens) => {
                    println!("VARIATION: {:?}", tokens);
                    current_game_variation_id += 1;
                    // TODO: recursively parse the variation
                    current_game_variation_id -= 1;
                }
                parser::PgnToken::Comment(comment) => {
                    println!("COMMENT: {}", comment);
                    // Assume the comment is for the last move
                    let last_move = game_result.moves.last_mut().unwrap();
                    last_move.annotation = Some(comment.to_string());
                }
                parser::PgnToken::Result(result) => {
                    println!("RESULT: {}", result);
                }
                _ => {
                    println!("UNKNOWN TOKEN: {:?}", token);
                    game_result
                        .errors
                        .push(format!("Unknown token: {:?}", token));
                }
            }
        });
    }

    LoadResult {
        games,
        success: true,
    }
}
