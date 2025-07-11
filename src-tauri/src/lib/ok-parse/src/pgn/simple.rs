use super::PgnToken;
use chumsky::prelude::*;

/// Parse a tag name (allowing more characters than just identifiers)
fn tag_name() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|&c: &char| c.is_alphanumeric() || c == ' ' || c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .labelled("tag name")
}

/// Parse a tag value (handling escaped quotes)
fn tag_value() -> impl Parser<char, String, Error = Simple<char>> {
    just('"')
        .ignore_then(
            choice((
                just('\\').ignore_then(just('"')).to('"'), // Handle escaped quotes
                filter(|&c| c != '"'),
            ))
            .repeated()
            .collect(),
        )
        .then_ignore(just('"'))
        .labelled("tag value")
}

/// Parse a complete tag [Name "Value"]
pub fn tag_parser() -> impl Parser<char, PgnToken, Error = Simple<char>> {
    just('[')
        .ignore_then(tag_name())
        .then(tag_value())
        .then_ignore(just(']'))
        .map(|(name, value)| PgnToken::Tag {
            name: name.trim().to_string(),
            value,
        })
        .labelled("tag")
}

/// Parse a move number (e.g., "1." or "1..." for half moves)
pub fn move_number_parser() -> impl Parser<char, PgnToken, Error = Simple<char>>
{
    text::int(10)
        .then_ignore(just('.').repeated().at_least(1))
        .map(|num: String| PgnToken::MoveNumber {
            number: num.parse().unwrap(),
        })
        .labelled("move number")
}

/// Parse a chess move
///
/// Just captures the notation of the entire move
/// does not validate any of the move's components
///
/// Does not include move suffix notation or annotation glyphs
pub fn chess_move_parser() -> impl Parser<char, PgnToken, Error = Simple<char>>
{
    filter(|&c: &char| c.is_alphanumeric() || "+#=x-".contains(c))
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|notation| PgnToken::Move { notation })
        .labelled("chess move")
}

/// Parse a game result
pub fn game_result_parser() -> impl Parser<char, PgnToken, Error = Simple<char>>
{
    choice((just("1-0"), just("0-1"), just("1/2-1/2"), just("*")))
        .map(|s: &str| PgnToken::Result {
            result: s.to_string(),
        })
        .labelled("game result")
}

/// Parse comments in curly braces
pub fn comment_parser() -> impl Parser<char, PgnToken, Error = Simple<char>> {
    just('{')
        .ignore_then(filter(|&c| c != '}').repeated().collect::<String>())
        .then_ignore(just('}'))
        .map(|text| PgnToken::Comment { text })
        .labelled("comment")
}

/// Parse Numeric Annotation Glyphs
pub fn nag_parser() -> impl Parser<char, PgnToken, Error = Simple<char>> {
    just('$')
        .ignore_then(text::int(10))
        .map(|num: String| PgnToken::NAG {
            code: num.parse().unwrap(),
        })
        .labelled("nag")
}
/// Parse move suffix notation like "?", "!", "??", "!!", "?!", "!?"
pub fn move_suffix_parser() -> impl Parser<char, PgnToken, Error = Simple<char>>
{
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
