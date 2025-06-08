use thiserror::Error;

/// Error type for the application
#[derive(Error, Debug, serde::Serialize)]
pub enum AppError {
    #[error("Parse error: {0}")]
    ParseError(String),
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
}
