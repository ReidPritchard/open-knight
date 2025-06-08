use super::simple::*;
use super::{PgnGame, PgnParseError, PgnToken};
use chumsky::prelude::*;

/// Parse variations in parentheses (recursive)
pub fn variation_parser() -> impl Parser<char, PgnToken, Error = Simple<char>> {
    recursive(|variation| {
        choice((
            tag_parser(),
            move_number_parser(),
            chess_move_parser(),
            comment_parser(),
            variation,
            nag_parser(),
        ))
        .padded()
        .repeated()
        .delimited_by(just('('), just(')'))
        .map(|moves| PgnToken::Variation { moves })
    })
    .labelled("variation")
}

/// Parse move suffix notation like "?", "!", "??", "!!", "?!", "!?"
pub fn move_suffix_parser() -> impl Parser<char, PgnToken, Error = Simple<char>> {
    choice((
        just("??"),
        just("!!"),
        just("?!"),
        just("!?"),
        just("?"),
        just("!"),
    ))
    .map(|suffix: &str| PgnToken::MoveSuffixNotation {
        suffix: suffix.to_string(),
    })
    .labelled("move suffix")
}

/// Parse a PGN string with enhanced metadata processing
pub fn parse_pgn_with_metadata(pgn: &str) -> Result<Vec<PgnGame>, PgnParseError> {
    let token_groups = super::simple::parse_pgn_games(pgn)?;

    let mut games = Vec::new();

    for tokens in token_groups {
        let game = PgnGame::from_tokens(tokens);

        // Validate that required tags are present
        if game.event().is_none() {
            return Err(PgnParseError::MissingRequiredTag {
                tag_name: "Event".to_string(),
            });
        }

        games.push(game);
    }

    Ok(games)
}

/// Parse and validate a single PGN game
pub fn parse_single_game(pgn: &str) -> Result<PgnGame, PgnParseError> {
    let tokens = super::parse_pgn_tokens(pgn)?;
    let game = PgnGame::from_tokens(tokens);

    // Basic validation
    validate_game(&game)?;

    Ok(game)
}

/// Validate a PGN game structure
pub fn validate_game(game: &PgnGame) -> Result<(), PgnParseError> {
    // Check for required tags
    let required_tags = ["Event", "Site", "Date", "Round", "White", "Black", "Result"];

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

/// Extract all moves from a game (excluding variations)
pub fn extract_main_line(game: &PgnGame) -> Vec<String> {
    let mut moves = Vec::new();

    for token in &game.moves {
        if let PgnToken::Move { notation } = token {
            moves.push(notation.clone());
        }
    }

    moves
}

/// Extract all variations from a game
pub fn extract_variations(tokens: &[PgnToken]) -> Vec<Vec<PgnToken>> {
    let mut variations = Vec::new();

    for token in tokens {
        match token {
            PgnToken::Variation { moves } => {
                variations.push(moves.clone());
                // Recursively extract nested variations
                variations.extend(extract_variations(moves));
            }
            _ => {}
        }
    }

    variations
}

/// Parse a FEN string from PGN tags
pub fn extract_fen(game: &PgnGame) -> Option<String> {
    game.get_tag("FEN").map(|s| s.to_string())
}

/// Extract time controls from PGN tags
pub fn extract_time_control(game: &PgnGame) -> Option<String> {
    game.get_tag("TimeControl").map(|s| s.to_string())
}

/// Extract ECO (Encyclopedia of Chess Openings) code
pub fn extract_eco(game: &PgnGame) -> Option<String> {
    game.get_tag("ECO").map(|s| s.to_string())
}
