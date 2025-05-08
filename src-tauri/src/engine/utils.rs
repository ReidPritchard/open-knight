//! Utility functions and common types

/// Comprehensive error types
#[derive(Debug, Clone)]
pub enum EngineError {
    ////// Engine Process Errors //////
    /// The engine process failed to start
    ProcessFailedToStart(String),
    /// The engine process failed to execute a command
    ProcessFailedToExecuteCommand(String),
    /// The engine/process is already running
    EngineAlreadyRunning(String),
    /// The engine/process is not running
    EngineNotRunning(String),
    /// The engine/process failed to kill
    ProcessFailedToKill(String),

    ////// Engine Protocol Errors //////
    /// The engine protocol failed to parse a line
    ProtocolFailedToParseLine(String),
    /// Invalid protocol type
    InvalidProtocolType(String),

    ////// Engine State Errors //////
    /// The engine state failed to update
    StateFailedToUpdate(String),

    ////// IO Errors //////
    /// The engine IO failed to read a line
    IoFailedToReadLine(String),
    /// The engine IO failed to write a line
    IoFailedToWriteLine(String),
    /// The engine IO failed to flush
    IoFailedToFlush(String),
    /// The engine IO failed to close
    IoFailedToClose(String),
    /// An invalid state was encountered
    InvalidState(String),
    /// The engine IO failed to join
    IoFailedToJoin(String),
    /// Output handler already started
    OutputHandlerAlreadyStarted(String),
}

impl From<std::io::Error> for EngineError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => EngineError::ProcessFailedToStart(e.to_string()),
            std::io::ErrorKind::PermissionDenied => {
                EngineError::ProcessFailedToStart(e.to_string())
            }
            std::io::ErrorKind::ConnectionRefused => {
                EngineError::ProcessFailedToStart(e.to_string())
            }
            _ => EngineError::IoFailedToReadLine(e.to_string()),
        }
    }
}

impl std::error::Error for EngineError {}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
