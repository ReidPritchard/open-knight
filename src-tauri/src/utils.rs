/// Error type for PGN parsing and processing
/// FIXME: Move this to the PGN parsing crate
/// and remove the unrelated error types
#[derive(Debug, serde::Serialize)]
pub enum AppError {
    ParseError(String),
    DatabaseError(String),
    SerializationError(String),
    ChessError(String),
    EngineError(String),
    SessionError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ParseError(e) => write!(f, "Parse error: {}", e),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            AppError::ChessError(e) => write!(f, "Chess error: {}", e),
            AppError::EngineError(e) => write!(f, "Engine error: {}", e),
            AppError::SessionError(e) => write!(f, "Session error: {}", e),
        }
    }
}
