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

/// Type for a single game+headers+errors result from the pgn loader
#[derive(Debug, Clone, Serialize)]
pub struct GameResult {
    pub id: i32,
    pub headers: Vec<(String, String)>,
    #[serde(skip)]
    pub game: Option<Chess>,
    pub moves: Vec<Move>,
    pub positions: Vec<crate::models::Position>,
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
            positions: Vec::new(),
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
            ((database::game::get_game_id_count() as i64 + games.len() as i64) + 1) as i32,
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
                    ((database::game::get_game_id_count() as i64 + games.len() as i64) + 1) as i32,
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
                    let mut before_move_fen: Option<String> = None;
                    let mut after_move_fen: Option<String> = None;

                    // See if we can play the move in the current game state
                    if let Some(ref mut game) = game_result.game {
                        // Get the FEN of the current position
                        before_move_fen = Some(
                            shakmaty::Position::board(game)
                                .board_fen(shakmaty::Position::promoted(game))
                                .to_string(),
                        );

                        // Convert the move string into a move object and try to play it
                        let mv_obj = san_obj.to_move(game).unwrap();

                        let is_valid_move = shakmaty::Position::is_legal(game, &mv_obj);
                        if !is_valid_move {
                            game_result.errors.push(format!("Invalid move: {}", mv));
                        } else {
                            // TODO: We probably need to handle variations here in order to avoid overwriting the game state
                            shakmaty::Position::play_unchecked(game, &mv_obj);
                            after_move_fen = Some(
                                shakmaty::Position::board(game)
                                    .board_fen(shakmaty::Position::promoted(game))
                                    .to_string(),
                            );
                        }
                    }

                    let mut before_move_position_id = database::position::get_position_id_by_fen(
                        before_move_fen.as_ref().unwrap(),
                    );
                    if before_move_position_id.is_none() {
                        println!(
                            "Creating new position for before move fen: {}",
                            before_move_fen.as_ref().unwrap()
                        );
                        // Create a new position
                        let new_position_id =
                            database::position::create_position(before_move_fen.as_ref().unwrap());
                        before_move_position_id = Some(new_position_id);
                    }

                    let mut after_move_position_id = database::position::get_position_id_by_fen(
                        after_move_fen.as_ref().unwrap(),
                    );
                    if after_move_position_id.is_none() {
                        println!(
                            "Creating new position for after move fen: {}",
                            after_move_fen.as_ref().unwrap()
                        );
                        // Create a new position
                        let new_position_id =
                            database::position::create_position(after_move_fen.as_ref().unwrap());
                        after_move_position_id = Some(new_position_id);
                    }

                    // Convert the move token string into a move object
                    let move_object = Move {
                        game_id: game_result.id,
                        move_san: mv.to_string(),
                        move_number: current_game_move_number as i32,
                        variation_order: Some(current_game_variation_id as i32),
                        parent_position_id: before_move_position_id.unwrap(),
                        child_position_id: after_move_position_id.unwrap(),
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
