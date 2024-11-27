use crate::models::{
    api::{APIGame, APIMove},
    db::{Game, Move, Position},
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
pub fn parsing_games_to_models(parsing_games: Vec<ParsingGame>) -> (Vec<Game>, Vec<Move>) {
    let games: Vec<Game> = parsing_games.iter().map(|pg| pg.game.clone()).collect();

    let moves: Vec<Move> = parsing_games
        .iter()
        .flat_map(|pg| pg.moves.clone())
        .collect();

    (games, moves)
}

/// Convert database moves to API moves
pub fn moves_to_api_moves(moves: Vec<(Move, Position, Position)>) -> Vec<APIMove> {
    moves.into_iter().map(APIMove::from).collect()
}

/// Convert database models to API games
pub fn to_api_games(games: Vec<Game>, moves: Vec<APIMove>) -> Vec<APIGame> {
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

/// Convert database models to explorer games
pub fn to_explorer_games(games: Vec<Game>) -> Vec<ExplorerGame> {
    games.into_iter().map(ExplorerGame::from).collect()
}

/// Convert database models to full games
pub fn to_full_games(games: Vec<Game>, moves: Vec<Move>) -> Vec<FullGame> {
    games
        .into_iter()
        .map(|game| {
            let game_moves = moves
                .iter()
                .filter(|m| m.game_id == game.id.unwrap())
                .cloned()
                .collect();
            FullGame {
                game,
                moves: game_moves,
            }
        })
        .collect()
}
