use crate::api_types::{APIGame, APIMove};
use crate::loader::GameResult;
use crate::models::{Game, Move, Position};

impl From<shakmaty::san::San> for Move {
    fn from(value: shakmaty::san::San) -> Self {
        Move {
            move_san: value.to_string(),
            ..Default::default()
        }
    }
}

impl From<GameResult> for Game {
    fn from(value: GameResult) -> Self {
        Game {
            id: Some(value.id),
            pgn: value.pgn,
            // Map headers to Game fields as needed
            player_white: value
                .headers
                .iter()
                .find(|(k, _)| k == "White")
                .map(|(_, v)| v.clone()),
            player_black: value
                .headers
                .iter()
                .find(|(k, _)| k == "Black")
                .map(|(_, v)| v.clone()),
            // Add other fields as needed
            ..Default::default()
        }
    }
}

pub fn game_results_to_games_and_moves(game_results: Vec<GameResult>) -> (Vec<Game>, Vec<Move>) {
    let games: Vec<Game> = game_results
        .iter()
        .map(|gr| Game::from(gr.clone()))
        .collect();
    let moves: Vec<Move> = game_results
        .iter()
        .flat_map(|gr| {
            gr.moves.iter().map(move |m| Move {
                game_id: gr.id,
                ..m.clone()
            })
        })
        .collect();
    (games, moves)
}

pub fn moves_to_api_moves(moves: Vec<(Move, Position, Position)>) -> Vec<APIMove> {
    moves.into_iter().map(APIMove::from).collect()
}

/**
 * Convert a vector of games and a vector of moves (from all games)
 * to a vector of API games.
 */
pub fn games_and_moves_to_api_games(games: Vec<Game>, moves: Vec<APIMove>) -> Vec<APIGame> {
    games
        .into_iter()
        .map(|game| {
            let game_moves = moves
                .iter()
                .filter(|m| m.game_move.game_id == game.id.unwrap())
                .cloned()
                .collect();
            APIGame::from((game, game_moves))
        })
        .collect()
}
