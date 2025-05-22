use crate::engine::utils::EngineError;
use std::fmt;

use super::events::LifecycleEvent;
use super::state::EngineState;

pub mod uci_protocol;

/// Trait for parsing engine output into engine events
pub trait ProtocolParser: Send + Sync {
    type State: EngineState;
    type Output;

    /// Parse a line of engine output into an engine event
    fn parse_line(&self, line: &str) -> Result<Self::Output, EngineError>;

    /// Get the name of the protocol
    fn protocol_name(&self) -> &'static str;
}

/// The output of the parser
pub enum ParserOutput<S: EngineState> {
    /// A state update
    StateUpdate(S::Update),
    /// A lifecycle event
    LifecycleEvent(LifecycleEvent),
    /// No update
    NoUpdate,
}

/// A generic type for a parser that implements ProtocolParser
pub type ProtocolParserType<S> = Box<dyn ProtocolParser<State = S, Output = ParserOutput<S>>>;

/// Protocol-agnostic chess command types to be used with ProtocolComposer
#[derive(Debug, Clone)]
pub enum EngineCommand {
    /// Raw command string to be sent directly (after protocol formatting)
    Raw(String),
    /// Check if engine is ready
    IsReady,
    /// Start a new game
    NewGame,
    /// Set engine position using FEN string and/or moves
    SetPosition {
        fen: Option<String>,
        moves: Option<Vec<String>>,
    },
    /// Start analysis with optional depth and time constraints
    StartAnalysis {
        depth: Option<u32>,
        movetime: Option<u32>,
        nodes: Option<u64>,
        multipv: Option<u32>,
        searchmoves: Option<Vec<String>>,
    },
    /// Stop ongoing analysis
    StopAnalysis,
    /// Set engine options
    SetOption { name: String, value: OptionValue },
    /// Quit the engine
    Quit,
}

/// Possible option value types for engine configuration
#[derive(Debug, Clone)]
pub enum OptionValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl fmt::Display for OptionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionValue::String(s) => write!(f, "{}", s),
            OptionValue::Integer(i) => write!(f, "{}", i),
            OptionValue::Float(fl) => write!(f, "{}", fl),
            OptionValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}

/// A protocol composer is used for formatting commands to be sent to the engine
///
/// This abstracts the protocol details away from consumers, who can use
/// protocol-agnostic commands that get translated to the specific protocol format.
pub trait ProtocolComposer: Send + Sync {
    /// Format a generic engine command into the specific protocol format
    fn compose(&self, command: EngineCommand) -> Result<String, EngineError>;

    /// Get the protocol name (e.g., "UCI", "CECP", etc.)
    fn protocol_name(&self) -> &str;

    /// Check if a specific feature is supported by this protocol
    fn supports_feature(&self, feature: &str) -> bool;

    /// Get the initial command to send to the engine
    ///
    /// Used to initialize the engine/protocol (ex. `uci` for UCI)
    fn initial_command(&self) -> Result<EngineCommand, EngineError>;
}

/// A generic type for a composer that implements ProtocolComposer
pub type ProtocolComposerType = Box<dyn ProtocolComposer>;
