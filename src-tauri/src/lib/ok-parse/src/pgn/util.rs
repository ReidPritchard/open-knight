use chumsky::prelude::*;

/// Parse a string token surrounded by quotes
pub fn quoted_string() -> impl Parser<char, String, Error = Simple<char>> {
    just('"')
        .ignore_then(
            choice((
                just('\\').ignore_then(just('"')).to('"'), // Handle escaped quotes
                just('\\').ignore_then(just('\\')).to('\\'), // Handle escaped backslashes
                filter(|&c| c != '"' && c != '\\'),
            ))
            .repeated()
            .collect(),
        )
        .then_ignore(just('"'))
        .labelled("quoted string")
}

/// Parse a whitespace-separated word
pub fn word() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|c: &char| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .padded()
}

/// Parse a number
pub fn number() -> impl Parser<char, u32, Error = Simple<char>> {
    text::int(10).padded().try_map(|s: String, span| {
        s.parse::<u32>()
            .map_err(|e| Simple::custom(span, format!("Invalid number: {}", e)))
    })
}

/// Parse a move in algebraic notation
pub fn algebraic_move() -> impl Parser<char, String, Error = Simple<char>> {
    choice((
        // Castling moves
        just("O-O-O").map(|s: &str| s.to_string()),
        just("O-O").map(|s: &str| s.to_string()),
        // Regular moves
        filter(|&c: &char| c.is_alphanumeric() || "+#=x-".contains(c))
            .repeated()
            .at_least(1)
            .collect::<String>(),
    ))
    .padded()
    .labelled("algebraic move")
}

/// Parse a sequence of moves until a non-move token
pub fn move_sequence() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    algebraic_move().repeated().collect()
}

/// Parse bracketed content (for tags)
pub fn bracketed<T>(
    inner: impl Parser<char, T, Error = Simple<char>>,
) -> impl Parser<char, T, Error = Simple<char>> {
    just('[').ignore_then(inner).then_ignore(just(']')).padded()
}

/// Parse parenthesized content (for variations)
pub fn parenthesized<T>(
    inner: impl Parser<char, T, Error = Simple<char>>,
) -> impl Parser<char, T, Error = Simple<char>> {
    just('(').ignore_then(inner).then_ignore(just(')')).padded()
}

/// Parse content in curly braces (for comments)
pub fn braced<T>(
    inner: impl Parser<char, T, Error = Simple<char>>,
) -> impl Parser<char, T, Error = Simple<char>> {
    just('{').ignore_then(inner).then_ignore(just('}')).padded()
}

/// Skip whitespace and comments
pub fn skip_whitespace_and_comments() -> impl Parser<char, (), Error = Simple<char>> {
    choice((
        filter(|c: &char| c.is_whitespace()).to(()),
        just(';')
            .ignore_then(filter(|&c| c != '\n').repeated())
            .to(()), // Line comments
    ))
    .repeated()
    .to(())
}

/// Validate a chess move format
pub fn is_valid_move_format(move_str: &str) -> bool {
    if move_str.is_empty() {
        return false;
    }

    // Check for castling
    if matches!(move_str, "O-O" | "O-O-O" | "0-0" | "0-0-0") {
        return true;
    }

    // Basic move format validation
    let chars: Vec<char> = move_str.chars().collect();

    // Should start with a piece or file
    if !chars[0].is_ascii_uppercase() && !chars[0].is_ascii_lowercase() {
        return false;
    }

    // Should contain only valid chess notation characters
    chars
        .iter()
        .all(|&c| c.is_alphanumeric() || "+#=x-".contains(c))
}

/// Extract the file (column) from a square notation like "e4"
pub fn extract_file(square: &str) -> Option<char> {
    square.chars().next().filter(|c| c.is_ascii_lowercase())
}

/// Extract the rank (row) from a square notation like "e4"
pub fn extract_rank(square: &str) -> Option<u8> {
    square.chars().nth(1)?.to_digit(10).map(|d| d as u8)
}

/// Check if a string represents a valid square (like "e4", "a1", etc.)
pub fn is_valid_square(square: &str) -> bool {
    if square.len() != 2 {
        return false;
    }

    let chars: Vec<char> = square.chars().collect();
    matches!(chars[0], 'a'..='h') && matches!(chars[1], '1'..='8')
}

/// Parse a semicolon-separated list of values
pub fn semicolon_separated<T>(
    item: impl Parser<char, T, Error = Simple<char>>,
) -> impl Parser<char, Vec<T>, Error = Simple<char>> {
    item.separated_by(just(';').padded()).collect()
}
