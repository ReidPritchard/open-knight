use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use serde::Serialize;
use shakmaty::{fen::Fen, san::SanError, CastlingMode, Chess, Move, Position};

#[derive(Debug, Clone)]
pub struct LoadResult {
    pub headers: Vec<Vec<(String, String)>>,
    pub games: Vec<Chess>,
    pub pgns: Vec<String>, // The PGN of each game, in order
    pub errors: Vec<Vec<String>>,
    pub success: bool,
}

/// Simple type for a single game+headers+errors result from the pgn loader
#[derive(Debug, Clone, Serialize)]
pub struct GameResult {
    pub headers: Vec<(String, String)>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub game: Chess,
    pub pgn: String,
    pub errors: Vec<String>,
}

impl LoadResult {
    pub fn new() -> Self {
        LoadResult {
            headers: Vec::new(),
            games: Vec::new(),
            pgns: Vec::new(),
            errors: Vec::new(),
            success: true,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, index: usize) -> Option<GameResult> {
        if index >= self.games.len() {
            return None;
        }

        let game = self.games.get(index).unwrap();
        let headers = self.headers.get(index).unwrap();
        let errors = self.errors.get(index).unwrap();
        let pgn = self.pgns.get(index).unwrap();
        Some(GameResult {
            headers: headers.clone(),
            game: game.clone(),
            errors: errors.clone(),
            pgn: pgn.clone(),
        })
    }

    /// Returns a vector of GameResults
    pub fn get_game_results(&self) -> Vec<GameResult> {
        self.games
            .iter()
            .enumerate()
            .map(|(i, game)| {
                let headers = self.headers.get(i).unwrap().clone();
                let errors = self.errors.get(i).unwrap().clone();
                let pgn = self.pgns.get(i).unwrap().clone();
                GameResult {
                    headers,
                    game: game.clone(),
                    pgn,
                    errors,
                }
            })
            .collect()
    }
}

impl Visitor for LoadResult {
    type Result = LoadResult;

    fn begin_game(&mut self) {
        self.pgns.push(String::new());
        self.games.push(Chess::new());
        self.headers.push(vec![]);
        self.errors.push(vec![]);
    }

    fn header(&mut self, key: &[u8], value: RawHeader<'_>) {
        let game_index = self.games.len() - 1;

        let key = String::from_utf8_lossy(key).to_string();
        let value = value.decode_utf8().unwrap().to_string();

        // Make sure the game has a headers vector
        if self.headers.get(game_index).is_none() {
            // It really should so just panic
            panic!("Game headers vector not found");
        }

        self.headers[game_index].push((key.clone(), value.clone()));
        self.pgns[game_index].push_str(&format!("[{} \"{}\"]\n", key, value));

        // TODO: Support games from a non-standard starting position.
    }

    fn begin_variation(&mut self) -> Skip {
        // TODO: Support variations
        Skip(true)
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.success {
            let game_index = self.games.len() - 1;
            // For some reason, play_unchecked requires the game to be mutable,
            // but rust doesn't see that as "using" the mutable reference. So ignore the lint
            #[allow(unused_mut)]
            let mut current_game = self.games.get_mut(game_index).unwrap();

            // Convert the SAN to a Move
            let san = san_plus.san as shakmaty::san::San;
            let move_result: Result<Move, SanError> = san.to_move(current_game);

            // if move_result is an error, set self.success to false
            if move_result.is_err() {
                self.success = false;
                self.errors[game_index].push(format!(
                    "Error parsing move: {}",
                    move_result.err().unwrap()
                ));
                return;
            }

            let move_result = move_result.unwrap();

            // This essentially does what `current_game.play(&move_result)` does, but
            // it doesn't consume the current game. Meaning we don't have to clone it
            // to update it.

            // Check if the move is legal
            if !current_game.is_legal(&move_result) {
                self.success = false;
                self.errors[game_index].push(format!("Illegal move: {}", move_result.to_string()));
                return;
            }
            // Play the move
            current_game.play_unchecked(&move_result);

            // Update the PGN string
            self.pgns[game_index].push_str(&format!("{} ", move_result.to_string()));
        }
    }

    fn end_game(&mut self) -> LoadResult {
        self.clone()
    }
}

pub fn load_pgn(pgn: &str) -> LoadResult {
    let mut reader = BufferedReader::new(pgn.as_bytes());
    let mut load_result = LoadResult::new();
    reader.read_game(&mut load_result).unwrap();
    load_result
}
