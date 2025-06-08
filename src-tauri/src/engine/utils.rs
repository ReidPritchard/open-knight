//! Utility functions and common types

use serde::Serialize;

/// Comprehensive error types
#[derive(Debug, Clone, Serialize, thiserror::Error)]
pub enum EngineError {
    ////// Engine Process Errors //////
    /// The engine process failed to start
    #[error("The engine process failed to start: {0}")]
    ProcessFailedToStart(String),
    /// The engine process failed to execute a command
    #[error("The engine process failed to execute a command: {0}")]
    ProcessFailedToExecuteCommand(String),
    /// The engine/process is already running
    #[error("The engine/process is already running: {0}")]
    EngineAlreadyRunning(String),
    /// The engine/process is not running
    #[error("The engine/process is not running: {0}")]
    EngineNotRunning(String),
    /// The engine/process failed to kill
    #[error("The engine/process failed to kill: {0}")]
    ProcessFailedToKill(String),

    ////// Engine Protocol Errors //////
    /// The engine protocol failed to parse a line
    #[error("The engine protocol failed to parse a line: {0}")]
    ProtocolFailedToParseLine(String),
    /// Invalid protocol type
    #[error("Invalid protocol type: {0}")]
    InvalidProtocolType(String),

    ////// Engine State Errors //////
    /// The engine state failed to update
    #[error("The engine state failed to update: {0}")]
    StateFailedToUpdate(String),

    ////// IO Errors //////
    /// The engine IO failed to read a line
    #[error("The engine IO failed to read a line: {0}")]
    IoFailedToReadLine(String),
    /// The engine IO failed to write a line
    #[error("The engine IO failed to write a line: {0}")]
    IoFailedToWriteLine(String),
    /// The engine IO failed to flush
    #[error("The engine IO failed to flush: {0}")]
    IoFailedToFlush(String),
    /// The engine IO failed to close
    #[error("The engine IO failed to close: {0}")]
    IoFailedToClose(String),
    /// An invalid state was encountered
    #[error("An invalid state was encountered: {0}")]
    InvalidState(String),
    /// The engine IO failed to join
    #[error("The engine IO failed to join: {0}")]
    IoFailedToJoin(String),
    /// Output handler already started
    #[error("Output handler already started: {0}")]
    OutputHandlerAlreadyStarted(String),
}
