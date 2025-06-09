// Module for parsing PGN files
use chumsky::prelude::*;
use log::debug;
use serde::Serialize;

use crate::DEBUG;

pub mod complex;
pub mod simple;
pub mod util;

/// Represents a token in a PGN file
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "token_type")]
pub enum PgnToken {
    /// Move numbers like "1."
    #[serde(rename = "move_number")]
    MoveNumber { number: u32 },

    /// Chess moves like "e4" or "Nf3"
    #[serde(rename = "move")]
    Move { notation: String },

    /// Move suffix notation like "??", "?!", "!?", "!!"
    #[serde(rename = "move_suffix")]
    MoveSuffixNotation { suffix: String },

    /// Game result like "1-0", "0-1", "1/2-1/2", "*"
    #[serde(rename = "result")]
    Result { result: String },

    /// Metadata in square brackets like [Event "World Championship"]
    #[serde(rename = "tag")]
    Tag { name: String, value: String },

    /// Comments in curly braces like {This is a comment}
    #[serde(rename = "comment")]
    Comment { text: String },

    /// Variations in parentheses like (1.e4 e5 2.Nf3)
    #[serde(rename = "variation")]
    Variation { moves: Vec<PgnToken> },

    /// Numeric Annotation Glyphs like $1, $2, etc.
    #[serde(rename = "nag")]
    NAG { code: u8 },
}

impl std::fmt::Display for PgnToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgnToken::MoveNumber { number } => write!(f, "{}.", number),
            PgnToken::Move { notation } => write!(f, "{} ", notation),
            PgnToken::Result { result } => write!(f, "{}", result),
            PgnToken::Tag { name, value } => write!(f, "[{} \"{}\"]", name, value),
            PgnToken::Comment { text } => write!(f, "{{{}}}", text),
            PgnToken::Variation { moves } => write!(
                f,
                "({})",
                moves.iter().map(|t| t.to_string()).collect::<String>()
            ),
            PgnToken::NAG { code } => write!(f, "${}", code),
            PgnToken::MoveSuffixNotation { suffix } => write!(f, "{}", suffix),
        }
    }
}

/// Represents a complete PGN game with metadata and moves
#[derive(Debug, Clone, Serialize)]
pub struct PgnGame {
    /// Game metadata tags (all PgnToken::Tag tokens)
    pub tags: Vec<PgnToken>,
    /// Game moves and annotations
    pub moves: Vec<PgnToken>,
    /// Game result
    pub result: Option<String>,
}

impl PgnGame {
    /// Create a new PGN game from tokens
    pub fn from_tokens(tokens: Vec<PgnToken>) -> Self {
        let mut tags = Vec::new();
        let mut moves = Vec::new();
        let mut result = None;

        for token in tokens {
            match &token {
                PgnToken::Tag { .. } => tags.push(token),
                PgnToken::Result { result: r } => {
                    result = Some(r.clone());
                    moves.push(token);
                }
                _ => moves.push(token),
            }
        }

        Self {
            tags,
            moves,
            result,
        }
    }

    /// Get the value of a specific tag
    pub fn get_tag(&self, tag_name: &str) -> Option<&str> {
        self.tags.iter().find_map(|token| {
            if let PgnToken::Tag { name, value } = token {
                if name == tag_name {
                    Some(value.as_str())
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

/// Error type for PGN parsing failures
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize)]
#[serde(tag = "error_type")]
pub enum PgnParseError {
    /// Failed to parse the PGN content
    #[error("Parse failure:\n\tInput: '{input}'\n\tMessage: {message}")]
    ParseFailure { input: String, message: String },

    /// Invalid tag format
    #[error("Invalid tag format:\n\tTag: '{tag}'\n\tReason: {reason}")]
    InvalidTag { tag: String, reason: String },

    /// Invalid move format
    #[error("Invalid move format:\n\tMove: '{move_text}'\n\tReason: {reason}")]
    InvalidMove { move_text: String, reason: String },

    /// Missing required tag
    #[error("Missing required tag:\n\tTag: '{tag_name}'")]
    MissingRequiredTag { tag_name: String },

    /// Invalid game result
    #[error("Invalid game result:\n\tResult: '{result}'")]
    InvalidResult { result: String },

    /// Malformed variation
    #[error("Malformed variation:\n\tReason: {reason}")]
    MalformedVariation { reason: String },

    /// Unknown token
    #[error("Unknown token:\n\tToken: '{token}'")]
    UnknownToken { token: String },
}

/// Top-level parser for PGN tokens
fn pgn_parser() -> impl Parser<char, Vec<PgnToken>, Error = Simple<char>> {
    choice((
        simple::tag_parser(),
        simple::game_result_parser(),
        simple::move_number_parser(),
        simple::chess_move_parser(),
        simple::nag_parser(),
        simple::comment_parser(),
        complex::variation_parser(),
    ))
    .padded()
    .repeated()
}

/// Parse a PGN string into tokens
///
/// # Arguments
/// * `pgn` - A PGN string to parse (can contain multiple games)
///
/// # Returns
/// * `Result<Vec<PgnToken>, PgnParseError>` - A vector of parsed PGN tokens or an error
fn pgn_string_to_tokens(pgn: &str) -> Result<Vec<PgnToken>, PgnParseError> {
    // TODO: Improve error recovery and allow for partial parsing

    if DEBUG {
        let (tokens, errors) = pgn_parser().parse_recovery_verbose(pgn);
        tokens.ok_or_else(|| PgnParseError::ParseFailure {
            input: pgn.to_string(),
            message: errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        })
    } else {
        pgn_parser()
            .parse(pgn)
            .map_err(|e| PgnParseError::ParseFailure {
                input: pgn.to_string(),
                message: e
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            })
    }
}

/// Parse a PGN string and group tokens by game
///
/// # Arguments
/// * `tokens` - A vector of PGN tokens to parse (can contain multiple games)
///
/// # Returns
/// * `Result<Vec<PgnGame>, PgnParseError>` - A vector of parsed PGN games or an error
fn pgn_tokens_to_games(tokens: Vec<PgnToken>) -> Result<Vec<PgnGame>, Vec<PgnParseError>> {
    let mut games = Vec::new();
    let mut current_game = Vec::new();
    let mut parse_errors = Vec::new();

    for token in tokens {
        if let PgnToken::Tag { name, .. } = &token {
            // Use the "Event" tag to determine the start of a new game
            // this should always be the first tag in the game (I think)
            if name == "Event" && !current_game.is_empty() {
                let game = PgnGame::from_tokens(current_game);
                match complex::validate_game(&game) {
                    Ok(_) => games.push(game),
                    Err(e) => parse_errors.push(e),
                }
                current_game = Vec::new();
            }
        }

        current_game.push(token);
    }

    if !current_game.is_empty() {
        let game = PgnGame::from_tokens(current_game);
        match complex::validate_game(&game) {
            Ok(_) => games.push(game),
            Err(e) => parse_errors.push(e),
        }
    }

    if !parse_errors.is_empty() {
        return Err(parse_errors);
    }

    Ok(games)
}

/// Public facade for parsing PGNs
///
/// # Arguments
/// * `pgn` - A PGN string to parse (can contain multiple games)
///
/// # Returns
/// * `Result<Vec<PgnGame>, Vec<PgnParseError>>` - A vector of parsed PGN games or an error
pub fn parse_pgn_games(pgn: &str) -> Result<Vec<PgnGame>, Vec<PgnParseError>> {
    if pgn.trim().is_empty() {
        return Err(vec![PgnParseError::ParseFailure {
            input: pgn.to_string(),
            message: "PGN string is empty".to_string(),
        }]);
    }

    debug!("Parsing PGN...");

    let tokens = pgn_string_to_tokens(pgn).map_err(|e| vec![e])?;

    debug!("\tParsed {} tokens", tokens.len());

    let games = pgn_tokens_to_games(tokens)?;

    debug!("\tParsed {} games", games.len());
    debug!("Parsing complete");

    Ok(games)
}
