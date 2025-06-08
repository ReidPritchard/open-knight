// Module for parsing PGN files
use chumsky::prelude::*;
use std::error::Error;
use std::fmt;

const DEBUG: bool = true;

#[derive(Debug, Clone)]
pub enum PgnToken {
    MoveNumber(u32),            // Move numbers like "1."
    Move(String),               // Chess moves like "e4" or "Nf3"
    MoveSuffixNotation(String), // Move suffix notation like "??", "?!", "!?", "!!"
    Result(String),             // Game result like "1-0", "0-1", "1/2-1/2", "*"
    Tag(String, String),        // Metadata in square brackets like [Event "World Championship"]
    Comment(String),            // Comments in curly braces like {This is a comment}
    Variation(Vec<PgnToken>),   // Variations in parentheses like (1.e4 e5 2.Nf3)
    NAG(u8),                    // Numeric Annotation Glyphs like $1, $2, etc.
}

#[derive(Debug)]
pub struct PgnError(Vec<Simple<char>>);

impl fmt::Display for PgnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PGN parse errors: {:?}", self.0)
    }
}

impl Error for PgnError {}

fn parser() -> impl Parser<char, Vec<PgnToken>, Error = Simple<char>> {
    // Parse a tag name (allowing more characters than just identifiers)
    let tag_name = filter(|&c: &char| c.is_alphanumeric() || c == ' ' || c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .labelled("tag name");

    // Parse a tag value (handling escaped quotes)
    let tag_value = just('"')
        .ignore_then(
            choice((
                just('\\').ignore_then(just('"')).to('"'), // Handle escaped quotes
                filter(|&c| c != '"'),
            ))
            .repeated()
            .collect(),
        )
        .then_ignore(just('"'))
        .labelled("tag value");

    // Parse a complete tag [Name "Value"]
    let tag = just('[')
        .ignore_then(tag_name)
        .then(tag_value)
        .then_ignore(just(']'))
        .map(|(name, value)| PgnToken::Tag(name.trim().to_string(), value));

    // Parse a move number (e.g., "1." or "1..." (chess.com uses this for half moves, not sure if it's standard))
    let move_number = text::int(10)
        .then_ignore(just('.').repeated().at_least(1).collect::<String>())
        .map(|num: String| PgnToken::MoveNumber(num.parse().unwrap()));

    // Parse a chess move (standard PGN format)
    let chess_move = filter(|&c: &char| c.is_alphanumeric() || "+#=x-".contains(c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(PgnToken::Move);

    // Parse a game result
    let game_result = choice((just("1-0"), just("0-1"), just("1/2-1/2"), just("*")))
        .map(|s: &str| PgnToken::Result(s.to_string()));

    // Parse comments in curly braces
    let comment = just('{')
        .ignore_then(filter(|&c| c != '}').repeated().collect::<String>())
        .then_ignore(just('}'))
        .map(PgnToken::Comment);

    // Parse variations in parentheses (recursive)
    let variation = recursive(|variation| {
        choice((
            tag.clone(),
            move_number.clone(),
            chess_move.clone(),
            comment.clone(),
            variation,
        ))
        .padded()
        .repeated()
        .delimited_by(just('('), just(')'))
        .map(PgnToken::Variation)
    });

    // Parse Numeric Annotation Glyphs
    let nag = just('$')
        .ignore_then(text::int(10))
        .map(|num: String| PgnToken::NAG(num.parse().unwrap()));

    // Combine all parsers
    choice((
        tag,
        game_result,
        move_number,
        chess_move,
        comment,
        variation,
        nag,
    ))
    .padded()
    .repeated()
}

pub fn parse_pgn(pgn: &str) -> Result<Vec<PgnToken>, PgnError> {
    if DEBUG {
        let (tokens, errors) = parser().parse_recovery_verbose(pgn);
        tokens.ok_or(PgnError(errors))
    } else {
        parser().parse(pgn).map_err(PgnError)
    }
}

/**
 * A function that parses a PGN into tokens and groups them by game
 *
 * @param pgn - The PGN to parse
 * @returns A vector of "games", where each game is a vector of tokens
 */
pub fn parse_pgn_games(pgn: &str) -> Result<Vec<Vec<PgnToken>>, PgnError> {
    let tokens = parse_pgn(pgn)?;

    let mut games = Vec::new();
    let mut current_game = Vec::new();

    for token in tokens {
        if let PgnToken::Tag(name, _) = &token {
            // Use the "Event" tag to determine the start of a new game
            // this should always be the first tag in the game (I think)
            if name == "Event" && !current_game.is_empty() {
                games.push(current_game);
                current_game = Vec::new();
            }
        }

        current_game.push(token);
    }

    if !current_game.is_empty() {
        games.push(current_game);
    }

    Ok(games)
}
