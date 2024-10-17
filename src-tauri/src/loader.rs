use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use serde::Serialize;
use shakmaty::{san::SanError, Chess, Move, Position};

#[derive(Debug, Clone)]
pub struct LoadResult {
    pub headers: Vec<Vec<(String, String)>>,
    pub games: Vec<Chess>,
    pub pgns: Vec<String>, // The PGN of each game
    pub ids: Vec<String>,  // The ID of each game
    pub errors: Vec<Vec<String>>,
    pub success: bool,
}

/// Simple type for a single game+headers+errors result from the pgn loader
#[derive(Debug, Clone, Serialize)]
pub struct GameResult {
    pub id: String,
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
            ids: Vec::new(),
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

        let id = self.ids.get(index).unwrap();
        let game = self.games.get(index).unwrap();
        let headers = self.headers.get(index).unwrap();
        let errors = self.errors.get(index).unwrap();
        let pgn = self.pgns.get(index).unwrap();
        Some(GameResult {
            id: id.clone(),
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
                let id = self.ids.get(i).unwrap().clone();
                let headers = self.headers.get(i).unwrap().clone();
                let errors = self.errors.get(i).unwrap().clone();
                let pgn = self.pgns.get(i).unwrap().clone();
                GameResult {
                    id,
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
        // Use sequential ids for each game
        let id = self.ids.len();
        self.ids.push(id.to_string());

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

    fn end_headers(&mut self) -> Skip {
        self.pgns[self.games.len() - 1].push_str("\n");
        Skip(true)
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
            let full_move_number = current_game.fullmoves().get();

            // If the move number is odd, it's a black move
            // if even, it's a white move and we need to add the move number
            // to the PGN string
            let pgn_move_string = if full_move_number % 2 == 1 {
                // This move is by white (since the move count is now odd)
                // So we need to prepend the move number to the PGN string
                format!("{} {}", full_move_number, move_result.to_string())
            } else {
                // This move is by black (since the move count is now even)
                // So we shouldn't prepend the move number to the PGN string
                move_result.to_string()
            };
            println!("Adding move: {}", pgn_move_string);

            self.pgns[game_index].push_str(&format!("{} ", pgn_move_string));
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
