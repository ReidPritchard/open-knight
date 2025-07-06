use super::simple::*;
use super::{PgnGame, PgnParseError, PgnToken};
use chumsky::prelude::*;

/// Parse variations in parentheses (recursive)
pub fn variation_parser() -> impl Parser<char, PgnToken, Error = Simple<char>> {
    recursive(|variation| {
        choice((
            move_number_parser(),
            chess_move_parser(),
            move_suffix_parser(),
            nag_parser(),
            comment_parser(),
            variation,
        ))
        .padded()
        .repeated()
        .delimited_by(just('('), just(')'))
        .map(|moves| PgnToken::Variation { moves })
    })
    .labelled("variation")
}

/// Validate a PGN game structure
pub fn validate_game(game: &PgnGame) -> Result<(), PgnParseError> {
    // Check for required tags
    let required_tags =
        ["Event", "Site", "Date", "Round", "White", "Black", "Result"];

    for tag_name in &required_tags {
        if game.get_tag(tag_name).is_none() {
            return Err(PgnParseError::MissingRequiredTag {
                tag_name: tag_name.to_string(),
            });
        }
    }

    // Validate result format
    if let Some(result) = &game.result {
        match result.as_str() {
            "1-0" | "0-1" | "1/2-1/2" | "*" => {}
            _ => {
                return Err(PgnParseError::InvalidResult {
                    result: result.clone(),
                })
            }
        }
    }

    Ok(())
}
