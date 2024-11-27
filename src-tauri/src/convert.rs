use crate::models::{
    api::{APIGame, APIMove},
    db::{Game, Header, Move, Position},
    game::{ExplorerGame, FullGame, ParsingGame},
};

impl From<shakmaty::san::San> for Move {
    fn from(value: shakmaty::san::San) -> Self {
        Move {
            move_san: value.to_string(),
            ..Default::default()
        }
    }
}

/// Convert a vector of parsing games to database models
pub fn parsing_games_to_models(
    parsing_games: Vec<ParsingGame>,
) -> (Vec<Game>, Vec<Move>, Vec<Header>) {
    let games: Vec<Game> = parsing_games.iter().map(|pg| pg.game.clone()).collect();

    let moves: Vec<Move> = parsing_games
        .iter()
        .flat_map(|pg| pg.moves.clone())
        .collect();

    let headers: Vec<Header> = parsing_games
        .iter()
        .flat_map(|pg| pg.headers.clone())
        .collect();

    (games, moves, headers)
}

/// Convert database moves to API moves
pub fn moves_to_api_moves(moves: Vec<(Move, Position, Position)>) -> Vec<APIMove> {
    moves.into_iter().map(APIMove::from).collect()
}

/// Convert database models to API games
pub fn to_api_games(games: Vec<Game>, moves: Vec<APIMove>, headers: Vec<Header>) -> Vec<APIGame> {
    games
        .into_iter()
        .map(|game| {
            let game_moves = moves
                .iter()
                .filter(|m| m.game_move.game_id == game.id.unwrap())
                .cloned()
                .collect();
            let game_headers = headers
                .iter()
                .filter(|h| h.game_id == game.id.unwrap())
                .cloned()
                .collect();
            APIGame::from((game, game_moves, game_headers))
        })
        .collect()
}

/// Convert database models to explorer games
pub fn to_explorer_games(games: Vec<Game>, headers: Vec<Header>) -> Vec<ExplorerGame> {
    games
        .into_iter()
        .map(|game| {
            let game_headers = headers
                .iter()
                .filter(|h| h.game_id == game.id.unwrap())
                .cloned()
                .collect();
            ExplorerGame::from((game, game_headers))
        })
        .collect()
}

/// Convert database models to full games
pub fn to_full_games(games: Vec<Game>, moves: Vec<Move>, headers: Vec<Header>) -> Vec<FullGame> {
    games
        .into_iter()
        .map(|game| {
            let game_moves = moves
                .iter()
                .filter(|m| m.game_id == game.id.unwrap())
                .cloned()
                .collect();
            let game_headers = headers
                .iter()
                .filter(|h| h.game_id == game.id.unwrap())
                .cloned()
                .collect();
            FullGame {
                game,
                moves: game_moves,
                headers: game_headers,
            }
        })
        .collect()
}
