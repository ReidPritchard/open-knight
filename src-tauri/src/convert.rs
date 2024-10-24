use crate::loader::GameResult;
use crate::models::{Game, Move};

/// Convert a Vec<GameResult> to a (Vec<Game>, Vec<Move>)
/// This allows easier insertion into the database
pub fn convert_to_games(game_results: Vec<GameResult>) -> (Vec<Game>, Vec<Move>) {
    let mut games = vec![];
    let mut moves = vec![];

    for game_result in game_results {
        games.push(Game {
            id: game_result.id,
            pgn: game_result.pgn,
            ..Default::default()
        });

        for game_move in game_result.game.moves() {
            moves.push(Move {
                game_id: game_result.id,
                san: game_move.san().unwrap().to_string(),
                ..Default::default()
            });
        }
    }

    (games, moves)
}
