// Module for parsing PGN files
use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum PgnToken {
    MoveNumber(u32),          // Move numbers like "1."
    Move(String),             // Chess moves like "e4" or "Nf3"
    Result(String),           // Game result like "1-0", "0-1", "1/2-1/2", "*"
    Tag(String, String),      // Metadata in square brackets like [Event "World Championship"]
    Comment(String),          // Comments in curly braces like {This is a comment}
    Variation(Vec<PgnToken>), // Variations in parentheses like (1.e4 e5 2.Nf3)
}

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
        .map(|(name, value)| PgnToken::Tag(name, value))
        .recover_with(skip_then_retry_until([']']));

    // Parse a move number (e.g., "1.")
    let move_number = text::int(10)
        .then_ignore(just('.'))
        .map(|num: String| PgnToken::MoveNumber(num.parse().unwrap()));

    // Parse a chess move (standard PGN format)
    let chess_move = filter(|&c: &char| c.is_alphanumeric() || "+#=x".contains(c))
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

    // Combine all parsers
    choice((
        tag,
        move_number,
        chess_move,
        game_result,
        comment,
        variation,
    ))
    .padded()
    .repeated()
}

pub fn parse_pgn(pgn: &str) -> Result<Vec<PgnToken>, Vec<Simple<char>>> {
    parser().parse(pgn)
}
