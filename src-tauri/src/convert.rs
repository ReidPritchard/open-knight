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

pub fn convert_to_games(game_results: Vec<GameResult>) -> (Vec<Game>, Vec<Move>) {
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

pub fn convert_to_game_results(
    games: Vec<Game>,
    moves: Vec<Move>,
    positions: Vec<Position>,
) -> Vec<GameResult> {
    let mut game_map: std::collections::HashMap<i32, GameResult> = games
        .into_iter()
        .map(|game| {
            (
                game.id.unwrap(),
                GameResult {
                    id: game.id.unwrap(),
                    pgn: game.pgn.clone(),
                    moves: Vec::new(),
                    positions: Vec::new(),
                    headers: vec![],
                    errors: vec![],
                    game: None, // Not needed now as we already have moves
                },
            )
        })
        .collect();

    for m in moves {
        if let Some(game_result) = game_map.get_mut(&m.game_id) {
            game_result.moves.push(m);
        }
    }

    for p in positions {
        if let Some(game_result) = game_map.get_mut(&p.game_id) {
            game_result.positions.push(p);
        }
    }

    game_map.into_values().collect()
}
