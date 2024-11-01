use serde::Serialize;
use shakmaty::{Chess, Outcome, Position};

use crate::{models::Move, parser};

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

impl LoadResult {
    pub fn new() -> Self {
        LoadResult {
            games: Vec::new(),
            success: true,
        }
    }

    pub fn get(&self, index: usize) -> Option<&GameResult> {
        self.games.get(index)
    }

    pub fn get_game_results(&self) -> &Vec<GameResult> {
        &self.games
    }
}

// impl Visitor for LoadResult {
//     type Result = ();

//     fn begin_game(&mut self) {
//         println!("BEGIN GAME ------------------------------");
//         let id = self.games.len() as i32;
//         self.games.push(GameResult {
//             id,
//             headers: Vec::new(),
//             game: Some(Chess::new()),
//             moves: Vec::new(),
//             pgn: String::new(),
//             errors: Vec::new(),
//         });
//     }

//     fn begin_headers(&mut self) {
//         println!("BEGIN HEADERS ------------------------------");
//     }

//     fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
//         if let Some(game_result) = self.games.last_mut() {
//             let key = String::from_utf8_lossy(key).to_string();
//             let value = value.decode_utf8().unwrap().to_string();

//             game_result.headers.push((key.clone(), value.clone()));
//             game_result
//                 .pgn
//                 .push_str(&format!("[{} \"{}\"]\n", key, value));
//         }
//     }

//     fn end_headers(&mut self) -> Skip {
//         if let Some(game_result) = self.games.last_mut() {
//             game_result.pgn.push_str("\n");
//         }
//         println!("END HEADERS ------------------------------");
//         Skip(false)
//     }

//     fn begin_variation(&mut self) -> Skip {
//         println!("BEGIN VARIATION ------------------------------");
//         // TODO: Support variations
//         Skip(true)
//     }

//     fn san(&mut self, san_plus: SanPlus) {
//         println!("SAN: {}", san_plus.san);
//         if let Some(game_result) = self.games.last_mut() {
//             let current_turn = game_result.game.as_ref().unwrap().turn();
//             let full_move_number = game_result.game.as_ref().unwrap().fullmoves().get();

//             let san = san_plus.san as shakmaty::san::San;
//             match san.to_move(game_result.game.as_ref().unwrap()) {
//                 Ok(mv) => {
//                     if !game_result.game.as_ref().unwrap().is_legal(&mv) {
//                         self.success = false;
//                         game_result
//                             .errors
//                             .push(format!("Illegal move: {}", mv.to_string()));
//                         return;
//                     }
//                     game_result.game.as_mut().unwrap().play_unchecked(&mv);

//                     // PGN move string
//                     let pgn_move_string = if current_turn == shakmaty::Color::White {
//                         format!("{}. {} ", full_move_number, san.to_string())
//                     } else {
//                         format!("{} ", san.to_string())
//                     };
//                     game_result.pgn.push_str(&pgn_move_string);

//                     // Move object
//                     let fen = game_result
//                         .game
//                         .as_ref()
//                         .unwrap()
//                         .board()
//                         .board_fen(game_result.game.as_ref().unwrap().promoted())
//                         .to_string();

//                     let move_object = Move {
//                         id: game_result.moves.len() as i32,
//                         game_id: game_result.id,
//                         annotation: None,
//                         fen: Some(fen),
//                         move_number: full_move_number as i32,
//                         move_san: pgn_move_string,
//                         parent_variation_id: None,
//                         variation_id: Some(0),
//                         ..Default::default()
//                     };
//                     game_result.moves.push(move_object);
//                 }
//                 Err(err) => {
//                     self.success = false;
//                     game_result.errors.push(format!(
//                         "Error parsing move: {}\n{}",
//                         err,
//                         san.to_string()
//                     ));
//                     println!("Error parsing move: {}", err);
//                 }
//             }
//         }
//     }

//     fn end_game(&mut self) {
//         println!("END GAME ------------------------------");
//     }

//     fn comment(&mut self, _comment: pgn_reader::RawComment<'_>) {
//         println!("COMMENT ------------------------------");
//     }

//     fn outcome(&mut self, _outcome: Option<Outcome>) {
//         println!("OUTCOME ------------------------------");
//         //
//     }

//     fn end_variation(&mut self) {
//         println!("END VARIATION ------------------------------");
//     }
// }

pub fn load_pgn(pgn: &str) -> LoadResult {
    let tokens = parser::parse_pgn(pgn);

    println!("TOKENS: {:?}", tokens);

    // load_result
    LoadResult::new()
}
