pub mod pgn;
pub mod uci;

pub const DEBUG: bool = true;

/// Error types for parsing module
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("PGN parse error: {0}")]
    PgnParseError(#[from] pgn::PgnParseError),
    #[error("UCI parse error: {0}")]
    UciParseError(#[from] uci::UciParseError),
}
