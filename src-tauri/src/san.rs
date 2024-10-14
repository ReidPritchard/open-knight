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

#[derive(Debug, PartialEq)]
pub struct Move {
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
            "" => Ok(Piece::Pawn), // Pawn moves don't typically include a piece letter
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
pub fn parse_san(san: &str) -> Result<Move, ParseError> {
    // let mut san = san.trim();

    let san: String = san.to_string();

    // Initialize flags
    let mut check = false;
    let mut checkmate = false;

    // Check for check '+' or checkmate '#'
    if san.ends_with('#') {
        checkmate = true;
        san = &san[..san.len() - 1];
    } else if san.ends_with('+') {
        check = true;
        san = &san[..san.len() - 1];
    }

    // Handle castling
    if san == "O-O" || san == "0-0" {
        return Ok(Move {
            piece: Piece::King,
            from: None,
            to: Square {
                file: 'g',
                rank: 1, // Adjust rank for black if needed
            },
            capture: false,
            promotion: None,
            check,
            checkmate,
        });
    } else if san == "O-O-O" || san == "0-0-0" {
        return Ok(Move {
            piece: Piece::King,
            from: None,
            to: Square {
                file: 'c',
                rank: 1, // Adjust rank for black if needed
            },
            capture: false,
            promotion: None,
            check,
            checkmate,
        });
    }

    // Handle promotion (e.g., "=Q")
    let mut promotion = None;
    if let Some(eq_idx) = san.find('=') {
        if eq_idx + 1 >= san.len() {
            return Err(ParseError::InvalidSAN(
                "Missing promotion piece after '='".to_string(),
            ));
        }
        let promo_char = &san[eq_idx + 1..eq_idx + 2];
        promotion = Some(
            promo_char
                .parse()
                .map_err(|_| ParseError::InvalidPromotionPiece)?,
        );
        san = &san[..eq_idx]; // Remove promotion part from SAN
    }

    // The last two characters should be the destination square
    if san.len() < 2 {
        return Err(ParseError::InvalidSAN("Notation too short".to_string()));
    }
    let dest_square_str = &san[san.len() - 2..];
    let to = dest_square_str
        .parse::<Square>()
        .map_err(|_| ParseError::InvalidSquare)?;

    // Remove destination square from SAN
    san = &san[..san.len() - 2];

    // Capture indicator
    let mut capture = false;
    if let Some(pos) = san.rfind('x') {
        capture = true;
        // Remove the 'x' from SAN
        // san = [&san[..pos], &san[pos + 1..]].concat();
        san = san.split_at(pos).
    }

    let mut chars = san.chars().peekable();

    // Piece moved (if any)
    let mut piece = Piece::Pawn;
    if let Some(&c) = chars.peek() {
        if c.is_uppercase() && "KQRBN".contains(c) {
            piece = c.to_string().parse()?;
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

    Ok(Move {
        piece,
        from,
        to,
        capture,
        promotion,
        check,
        checkmate,
    })
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Ok(Move {
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
            Err(ParseError::InvalidSAN(
                "Unexpected character ' ' in disambiguation".to_string()
            ))
        );

        assert_eq!(
            parse_san("e8="),
            Err(ParseError::InvalidSAN(
                "Missing promotion piece after '='".to_string()
            ))
        );

        assert_eq!(parse_san("e8=Z"), Err(ParseError::InvalidPromotionPiece));

        assert_eq!(parse_san("e4e5"), Err(ParseError::InvalidSquare));

        assert_eq!(parse_san("Pe4"), Err(ParseError::InvalidPiece));
    }
}
