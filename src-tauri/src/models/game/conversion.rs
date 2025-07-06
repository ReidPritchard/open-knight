use ok_parse::pgn::{PgnGame, PgnToken};

use crate::models::{
    parse::pgn_tokens_to_move_tree, structs::ChessHeader, ChessPosition,
};

use super::structs::ChessGame;

/// Converts a PgnGame into a ChessGame
impl From<PgnGame> for ChessGame {
    fn from(pgn_game: PgnGame) -> Self {
        let mut chess_game = ChessGame::new_default();

        let pgn_tags = pgn_game.tags;
        let pgn_moves = pgn_game.moves;
        let pgn_result = pgn_game.result;

        // Parse and set headers/tags
        for tag in pgn_tags {
            if let PgnToken::Tag { name, value } = tag {
                match name.as_str() {
                    "Event" => {
                        let mut tournament =
                            chess_game.tournament.unwrap_or_default();
                        tournament.name = value;
                        chess_game.tournament = Some(tournament);
                    }
                    "Site" => {
                        let mut tournament =
                            chess_game.tournament.unwrap_or_default();
                        tournament.location = Some(value);
                        chess_game.tournament = Some(tournament);
                    }
                    "Date" => chess_game.date = value,
                    "Round" => {
                        // safely try to parse the round number
                        if let Ok(round) = value.parse::<i32>() {
                            chess_game.round = Some(round);
                        } else {
                            chess_game.round = None;
                        }
                    }
                    "White" => chess_game.white_player.name = value,
                    "Black" => chess_game.black_player.name = value,
                    "Result" => chess_game.result = value,
                    "ECO" => {
                        let mut opening =
                            chess_game.opening.unwrap_or_default();
                        opening.eco = Some(value);
                        chess_game.opening = Some(opening);
                    }
                    "Opening" => {
                        let mut opening =
                            chess_game.opening.unwrap_or_default();
                        opening.name = Some(value);
                        chess_game.opening = Some(opening);
                    }
                    "Variation" => {
                        let mut opening =
                            chess_game.opening.unwrap_or_default();
                        opening.variation = Some(value);
                        chess_game.opening = Some(opening);
                    }
                    "FEN" => chess_game.fen = Some(value),
                    "Variant" => chess_game.variant = value,
                    _ => {
                        // All unknown tags are added to the headers
                        chess_game.headers.push(ChessHeader {
                            id: None,
                            game_id: chess_game.id,
                            name,
                            value,
                        });
                    }
                }
            }
        }

        // Parse and set moves
        let variant = chess_game.variant.clone();
        let starting_position = chess_game
            .fen
            .clone()
            .map_or(ChessPosition::default(), |fen| {
                ChessPosition::from_fen(Some(fen), Some(variant)).unwrap()
            });
        chess_game.move_tree = pgn_tokens_to_move_tree(
            chess_game.id,
            starting_position,
            &pgn_moves,
        )
        .unwrap();

        // Set result
        chess_game.result = pgn_result.unwrap_or("*".to_string());

        chess_game
    }
}

/// Converts a ChessGame into a PGN string
impl From<ChessGame> for String {
    fn from(game: ChessGame) -> Self {
        let mut pgn = String::new();

        // Add standard tags
        pgn.push_str(&format!(
            "[Event \"{}\"]\n",
            game.tournament
                .as_ref()
                .map_or("Casual Game".to_string(), |t| t.name.clone())
        ));
        pgn.push_str(&format!(
            "[Site \"{}\"]\n",
            game.tournament
                .as_ref()
                .and_then(|t| t.location.as_ref())
                .map(String::as_str)
                .unwrap_or("?")
        ));
        pgn.push_str(&format!("[Date \"{}\"]\n", game.date));
        pgn.push_str(&format!(
            "[Round \"{}\"]\n",
            game.round.map_or("?".to_string(), |r| r.to_string())
        ));
        pgn.push_str(&format!("[White \"{}\"]\n", game.white_player.name));
        pgn.push_str(&format!("[Black \"{}\"]\n", game.black_player.name));
        pgn.push_str(&format!("[Result \"{}\"]\n", game.result));

        if let Some(ref opening) = game.opening {
            if let Some(ref eco) = opening.eco {
                pgn.push_str(&format!("[ECO \"{}\"]\n", eco));
            }
            if let Some(ref name) = opening.name {
                pgn.push_str(&format!("[Opening \"{}\"]\n", name));
            }
            if let Some(ref variation) = opening.variation {
                pgn.push_str(&format!("[Variation \"{}\"]\n", variation));
            }
        }

        if let Some(ref elo) = game.white_player.elo {
            pgn.push_str(&format!("[WhiteElo \"{}\"]\n", elo));
        }
        if let Some(ref elo) = game.black_player.elo {
            pgn.push_str(&format!("[BlackElo \"{}\"]\n", elo));
        }

        // Add any custom tags
        for tag in &game.tags {
            if !tag.starts_with("[Event ")
                && !tag.starts_with("[Site ")
                && !tag.starts_with("[Date ")
                && !tag.starts_with("[Round ")
                && !tag.starts_with("[White ")
                && !tag.starts_with("[Black ")
                && !tag.starts_with("[Result ")
                && !tag.starts_with("[ECO ")
                && !tag.starts_with("[Opening ")
                && !tag.starts_with("[Variation ")
                && !tag.starts_with("[WhiteElo ")
                && !tag.starts_with("[BlackElo ")
            {
                pgn.push_str(&format!("{}\n", tag));
            }
        }

        pgn.push('\n');

        // Add moves
        pgn.push_str(&game.move_tree.to_pgn_moves());
        pgn.push_str(&game.result);
        pgn
    }
}
