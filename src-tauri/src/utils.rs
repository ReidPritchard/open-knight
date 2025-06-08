use ok_parse::pgn::PgnParseError;
use serde::Serialize;
use thiserror::Error;

/// Error type for the application
#[derive(Error, Debug, Serialize)]
pub enum AppError {
    #[error(transparent)]
    ParseError(#[from] PgnParseError),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Chess error: {0}")]
    ChessError(String),

    #[error("Engine error: {0}")]
    EngineError(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("General error: {0}")]
    GeneralError(String),
}
