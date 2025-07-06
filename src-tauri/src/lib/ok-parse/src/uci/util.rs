use chumsky::prelude::*;

/// Parse a token (a word followed by whitespace)
pub fn token(s: &'static str) -> impl Parser<char, (), Error = Simple<char>> {
    just(s).padded().map(|_| ())
}

/// Parse an identifier (a sequence of non-whitespace characters)
pub fn identifier() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|c: &char| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .padded()
}

/// Parse a number
pub fn number() -> impl Parser<char, u64, Error = Simple<char>> {
    text::int(10)
        .padded()
        .map(|s: String| s.parse::<u64>().unwrap())
}

/// Parse a signed number
pub fn signed_number() -> impl Parser<char, i32, Error = Simple<char>> {
    just('-').or_not().then(text::int(10)).padded().map(
        |(sign, s): (Option<char>, String)| {
            let n = s.parse::<i32>().unwrap();
            if sign.is_some() {
                -n
            } else {
                n
            }
        },
    )
}

/// Parse a chess move (e.g., "e2e4")
pub fn chess_move() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|c: &char| c.is_alphanumeric())
        .repeated()
        .exactly(4) // Most moves are 4 chars, but promotions can be 5
        .or(filter(|c: &char| c.is_alphanumeric()).repeated().exactly(5))
        .collect::<String>()
        .padded()
}

/// Parse a sequence of chess moves
pub fn move_sequence() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    chess_move().repeated().collect()
}
