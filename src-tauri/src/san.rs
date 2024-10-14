use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq)]
pub struct Square {
    pub file: char,
    pub rank: u8,
}

#[derive(Debug, PartialEq)]
pub struct Disambiguation {
    pub file: Option<char>,
    pub rank: Option<u8>,
}

/// A single move in chess
///
/// Not called "move" as that name is taken by the std lib
#[derive(Debug, PartialEq)]
pub struct ChessMove {
    pub piece: Piece,
    pub from: Option<Disambiguation>,
    pub to: Square,
    pub capture: bool,
    pub promotion: Option<Piece>,
    pub check: bool,
    pub checkmate: bool,
}

// Custom error type for parsing errors
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidPiece,
    InvalidSquare,
    InvalidPromotionPiece,
    InvalidSAN(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidPiece => write!(f, "Invalid piece"),
            ParseError::InvalidSquare => write!(f, "Invalid square"),
            ParseError::InvalidPromotionPiece => write!(f, "Invalid promotion piece"),
            ParseError::InvalidSAN(s) => write!(f, "Invalid SAN notation: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

impl FromStr for Piece {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "K" => Ok(Piece::King),
            "Q" => Ok(Piece::Queen),
            "R" => Ok(Piece::Rook),
            "B" => Ok(Piece::Bishop),
            "N" => Ok(Piece::Knight),
            "P" => Ok(Piece::Pawn), // Can technically be included but nearly always omitted
            "" => Ok(Piece::Pawn),  // Pawn moves don't typically include a piece letter
            _ => Err(ParseError::InvalidPiece),
        }
    }
}

/// Unsure if it's better to just convert chars to string and use FromStr
/// or to implement a TryFrom for char. I'd think this is better for readability
/// and efficiency, but it's also more code and kinda redundant.
impl TryFrom<char> for Piece {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'K' => Ok(Piece::King),
            'Q' => Ok(Piece::Queen),
            'R' => Ok(Piece::Rook),
            'B' => Ok(Piece::Bishop),
            'N' => Ok(Piece::Knight),
            'P' => Ok(Piece::Pawn), // Can technically be included but nearly always omitted
            _ => Err(ParseError::InvalidPiece),
        }
    }
}

impl FromStr for Square {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseError::InvalidSquare);
        }
        let mut chars = s.chars();
        let file = chars.next().ok_or(ParseError::InvalidSquare)?;
        let rank_char = chars.next().ok_or(ParseError::InvalidSquare)?;
        let rank = rank_char.to_digit(10).ok_or(ParseError::InvalidSquare)? as u8;

        if file < 'a' || file > 'h' || rank < 1 || rank > 8 {
            return Err(ParseError::InvalidSquare);
        }

        Ok(Square { file, rank })
    }
}

/// Parse SAN notation into a Move
#[allow(dead_code)]
pub fn parse_san(san: &str, is_white: Option<bool>) -> Result<ChessMove, ParseError> {
    let san = san.trim();

    // First handle the empty case for sanity
    if san.is_empty() {
        return Err(ParseError::InvalidSAN("Empty input".to_string()));
    }

    // Setup our iterators
    // // There isn't a peekable double ended iterator so we will use two and
    // // just use next_back() to fake it
    // let mut chars = san.chars().peekable();
    // let mut rev_chars = san.chars().rev().peekable();
    let mut chars = san.chars().multipeek();

    // Check for check '+' or checkmate '#'
    // do this first so it works for castling
    let (check, checkmate) = match rev_chars.peek() {
        Some('+') => {
            // Remove the '+'
            rev_chars.next();
            chars.next_back();
            (true, false)
        }
        Some('#') => {
            // Remove the '#'
            rev_chars.next();
            chars.next_back();
            (false, true)
        }
        Some(c) => (false, false),
        None => return Err(ParseError::InvalidSAN("Empty input".to_string())),
    };

    // Handle castling
    // Check for 'O' or '0'
    if let Some(&first_char) = chars.peek() {
        if first_char == 'O' || first_char == '0' {
            // Remove the 'O'
            chars.next();
            rev_chars.next_back();

            // At this point we can assume it's a castling move
            // if the syntax is invalid, we should return an error
            // Meaning we don't need to peek and instead use next()
            // We also won't update the rev_chars iterator as we no longer need it
            if chars.next() != Some('-') {
                return Err(ParseError::InvalidSAN(
                    "Invalid castling syntax".to_string(),
                ));
            }

            // Check for next char 'O' or '0'
            if let Some(next_char) = chars.next() {
                if next_char != 'O' && next_char != '0' {
                    return Err(ParseError::InvalidSAN(
                        "Invalid castling syntax".to_string(),
                    ));
                }
            }

            // At this point we have a valid king side castling move
            // If the iterator is at the end, we can return the move
            let is_white = is_white.unwrap_or(true);
            let rank = if is_white { 1 } else { 8 };
            if chars.peek().is_none() {
                // kingside castling is always to g1 no matter the color
                return Ok(ChessMove {
                    piece: Piece::King,
                    from: None,
                    to: Square { file: 'g', rank },
                    capture: false,
                    promotion: None,
                    check,
                    checkmate,
                });
            }

            // Otherwise, we need to check for queenside castling

            // look for the '-'
            if chars.next() != Some('-') {
                return Err(ParseError::InvalidSAN(
                    "Invalid castling syntax".to_string(),
                ));
            }

            // Check for next char 'O' or '0'
            if let Some(next_char) = chars.next() {
                if next_char != 'O' && next_char != '0' {
                    return Err(ParseError::InvalidSAN(
                        "Invalid castling syntax".to_string(),
                    ));
                }
            }

            // Now, given there are no more chars, we can return the move
            if chars.peek().is_none() {
                return Ok(ChessMove {
                    piece: Piece::King,
                    from: None,
                    to: Square { file: 'c', rank },
                    capture: false,
                    promotion: None,
                    check,
                    checkmate,
                });
            }

            // If we reach this point, the syntax is invalid
            return Err(ParseError::InvalidSAN(
                "Invalid castling syntax".to_string(),
            ));
        }
    }

    // Handle promotion (e.g., "=Q")
    // if present, this will be the next characters in the reversed iterator
    let mut promotion = None;
    if let Some(&promo_char) = rev_chars.peek() {
        if let Ok(p) = Piece::try_from(promo_char) {
            promotion = Some(p);
            rev_chars.next();
            chars.next_back();
        }
    }

    // The last two characters should be the destination square
    if san.len() < 2 {
        return Err(ParseError::InvalidSAN("Notation too short".to_string()));
    }
    let dest_square_str = san.get(san.len() - 2..).unwrap();
    let to = dest_square_str
        .parse::<Square>()
        .map_err(|_| ParseError::InvalidSquare)?;

    // Remove destination square from SAN (last two characters)
    san.truncate(san.len() - 2);

    // Capture indicator
    let mut capture = false;
    if let Some(_pos) = san.rfind('x') {
        capture = true;
        // Remove the 'x' from SAN (I think always the last character...right?)
        assert!(san.ends_with('x'));
        san.pop();
    }

    // At this point `san` should be something like `"Nb"` or `"K"` since
    // captures, checks, promotions, and the destination square have been removed
    // so we are left with the optional piece moved and the optional disambiguation

    let mut chars = san.chars().peekable();

    // Parse the piece moved (optional)
    // Defaults to pawn if not specified
    let mut piece = Piece::Pawn;
    if let Some(&c) = chars.peek() {
        if let Ok(p) = Piece::try_from(c) {
            piece = p;
            chars.next();
        }
    }

    // Disambiguation (remaining characters)
    let mut from_file: Option<char> = None;
    let mut from_rank: Option<u8> = None;

    while let Some(c) = chars.next() {
        if c.is_lowercase() && 'a' <= c && c <= 'h' {
            if from_file.is_none() {
                from_file = Some(c);
            } else {
                return Err(ParseError::InvalidSAN(
                    "Multiple from files in disambiguation".to_string(),
                ));
            }
        } else if c.is_digit(10) && '1' <= c && c <= '8' {
            if from_rank.is_none() {
                from_rank = Some(c.to_digit(10).unwrap() as u8);
            } else {
                return Err(ParseError::InvalidSAN(
                    "Multiple from ranks in disambiguation".to_string(),
                ));
            }
        } else {
            return Err(ParseError::InvalidSAN(format!(
                "Unexpected character '{}' in disambiguation",
                c
            )));
        }
    }

    let from = if from_file.is_some() || from_rank.is_some() {
        Some(Disambiguation {
            file: from_file,
            rank: from_rank,
        })
    } else {
        None
    };

    Ok(ChessMove {
        piece,
        from,
        to,
        capture,
        promotion,
        check,
        checkmate,
    })
}

/// Generate SAN notation from a ChessMove
///
#[allow(dead_code)]
pub fn generate_san(chess_move: &ChessMove) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_from_str() {
        assert_eq!("K".parse::<Piece>(), Ok(Piece::King));
        assert_eq!("Q".parse::<Piece>(), Ok(Piece::Queen));
        assert_eq!("R".parse::<Piece>(), Ok(Piece::Rook));
        assert_eq!("B".parse::<Piece>(), Ok(Piece::Bishop));
        assert_eq!("N".parse::<Piece>(), Ok(Piece::Knight));
        assert_eq!("P".parse::<Piece>(), Ok(Piece::Pawn));
        assert_eq!("".parse::<Piece>(), Ok(Piece::Pawn));
        assert!("X".parse::<Piece>().is_err());
    }

    #[test]
    fn test_square_from_str() {
        assert_eq!("e4".parse::<Square>(), Ok(Square { file: 'e', rank: 4 }));
        assert_eq!("a1".parse::<Square>(), Ok(Square { file: 'a', rank: 1 }));
        assert_eq!("h8".parse::<Square>(), Ok(Square { file: 'h', rank: 8 }));
        assert!("i9".parse::<Square>().is_err());
        assert!("e0".parse::<Square>().is_err());
        assert!("x5".parse::<Square>().is_err());
        assert!("e".parse::<Square>().is_err());
        assert!("".parse::<Square>().is_err());
    }

    #[test]
    fn test_parse_san_positive() {
        assert_eq!(
            parse_san("e4"),
            Ok(ChessMove {
                piece: Piece::Pawn,
                from: None,
                to: Square { file: 'e', rank: 4 },
                capture: false,
                promotion: None,
                check: false,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("Nf3"),
            Ok(ChessMove {
                piece: Piece::Knight,
                from: None,
                to: Square { file: 'f', rank: 3 },
                capture: false,
                promotion: None,
                check: false,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("exd5"),
            Ok(ChessMove {
                piece: Piece::Pawn,
                from: Some(Disambiguation {
                    file: Some('e'),
                    rank: None
                }),
                to: Square { file: 'd', rank: 5 },
                capture: true,
                promotion: None,
                check: false,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("Nbd7"),
            Ok(ChessMove {
                piece: Piece::Knight,
                from: Some(Disambiguation {
                    file: Some('b'),
                    rank: None
                }),
                to: Square { file: 'd', rank: 7 },
                capture: false,
                promotion: None,
                check: false,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("R1e2"),
            Ok(ChessMove {
                piece: Piece::Rook,
                from: Some(Disambiguation {
                    file: None,
                    rank: Some(1)
                }),
                to: Square { file: 'e', rank: 2 },
                capture: false,
                promotion: None,
                check: false,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("e8=Q+"),
            Ok(ChessMove {
                piece: Piece::Pawn,
                from: None,
                to: Square { file: 'e', rank: 8 },
                capture: false,
                promotion: Some(Piece::Queen),
                check: true,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("O-O"),
            Ok(ChessMove {
                piece: Piece::King,
                from: None,
                to: Square { file: 'g', rank: 1 },
                capture: false,
                promotion: None,
                check: false,
                checkmate: false,
            })
        );

        assert_eq!(
            parse_san("O-O-O#"),
            Ok(ChessMove {
                piece: Piece::King,
                from: None,
                to: Square { file: 'c', rank: 1 },
                capture: false,
                promotion: None,
                check: false,
                checkmate: true,
            })
        );

        assert_eq!(
            parse_san("Rxh8=Q"),
            Ok(ChessMove {
                piece: Piece::Rook,
                from: None,
                to: Square { file: 'h', rank: 8 },
                capture: true,
                promotion: Some(Piece::Queen),
                check: false,
                checkmate: false,
            })
        );
    }

    #[test]
    fn test_parse_san_negative() {
        assert_eq!(
            parse_san(""),
            Err(ParseError::InvalidSAN("Notation too short".to_string()))
        );

        assert_eq!(
            parse_san("e"),
            Err(ParseError::InvalidSAN("Notation too short".to_string()))
        );

        assert_eq!(parse_san("e9"), Err(ParseError::InvalidSquare));

        assert_eq!(parse_san("i5"), Err(ParseError::InvalidSquare));

        assert_eq!(
            parse_san("RNBQKB1R w KQkq - 0 1"),
            Err(ParseError::InvalidSquare)
        );

        assert_eq!(
            parse_san("e8="),
            Err(ParseError::InvalidSAN(
                "Missing promotion piece after '='".to_string()
            ))
        );

        assert_eq!(parse_san("e8=Z"), Err(ParseError::InvalidPromotionPiece));

        assert_eq!(parse_san("e4t5"), Err(ParseError::InvalidSquare));
        assert_eq!(parse_san("e4e9"), Err(ParseError::InvalidSquare));

        assert_eq!(parse_san("Pe4"), Err(ParseError::InvalidPiece));
    }
}
