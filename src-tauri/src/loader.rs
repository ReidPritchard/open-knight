use serde::Serialize;
use shakmaty::Chess;

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
            game: None,
            moves: Vec::new(),
            pgn: String::new(),
            errors: Vec::new(),
        }
    }
}

impl LoadResult {
    pub fn new() -> Self {
        LoadResult {
            games: Vec::new(),
            success: true,
        }
    }

    pub fn get_game_results(&self) -> &Vec<GameResult> {
        &self.games
    }
}

pub fn load_pgn(pgn: &str) -> LoadResult {
    // Parse the pgn (returns a vector of tokens or errors)
    let tokens = parser::parse_pgn(pgn);

    // Convert the tokens into a game result object
    let mut game_result = GameResult::new();
    game_result.id = (database::get_game_id_count() + 1) as i32;

    // Tokens can contain multiple games, so we need to loop through them
    // and determine where each game starts and ends while parsing
    let mut games: Vec<GameResult> = Vec::new();
    if let Ok(tokens) = tokens {
        let mut game_id_move_count = 0;
        let mut game_started = false;
        tokens.iter().for_each(|token| {
            match token {
                parser::PgnToken::Tag(key, value) => {
                    // If we've already seen a game's moves, we should consider this a new game
                    if game_started {
                        println!("Pushing game to vector");
                        games.push(game_result.clone());
                        game_result = GameResult::new();
                        game_result.id = ((database::get_game_id_count() as i64
                            + games.len() as i64)
                            + 1) as i32;
                        game_started = false;
                    }

                    println!("TAG: {} = {}", key, value);
                    game_result
                        .headers
                        .push((key.to_string(), value.to_string()));
                }
                parser::PgnToken::Move(mv) => {
                    game_id_move_count += 1;
                    game_started = true;
                    println!("MOVE: {}", mv);
                    // Convert the move token string into a move object
                    let move_object = Move {
                        // Generate a new id for the move (move db count + number of moves in games loaded so far)
                        id: (database::get_move_id_count() + game_id_move_count + 1) as i32,
                        game_id: game_result.id,
                        move_san: mv.to_string(),
                        ..Default::default()
                    };
                    game_result.moves.push(move_object);
                }
                parser::PgnToken::MoveNumber(num) => {
                    println!("MOVE NUMBER: {}", num);
                }
                parser::PgnToken::Comment(comment) => {
                    println!("COMMENT: {}", comment);
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
