use ok_parse::pgn::{parse_pgn_games, PgnGame, PgnToken};
use sea_orm::sqlx::types::chrono;

use crate::{
    models::{
        parse::parse_pgn_tokens, ChessMoveTree, ChessOpening, ChessPlayer, ChessPosition,
        ChessTournament,
    },
    utils::AppError,
};

use super::structs::ChessGame;

/// Converts a PgnGame into a ChessGame
impl From<PgnGame> for ChessGame {
    fn from(_pgn_game: PgnGame) -> Self {
        let _chess_game = ChessGame::new_default();

        todo!()
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
        pgn.push_str(&format!("{}", game.result));
        pgn
    }
}
