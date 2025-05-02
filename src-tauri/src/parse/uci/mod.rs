use chumsky::prelude::*;
use complex::{info_token_parser, option_token_parser, parse_info_params, parse_option_params};
use serde::Serialize;
use simple::{
    bestmove_parser, copyprotection_parser, id_parser, readyok_parser, registration_parser,
    uciok_parser,
};

pub mod complex;
pub mod simple;
pub mod util;

/// Represents a response from the engine to the GUI
#[derive(Serialize)]
#[serde(tag = "message_type")]
pub enum EngineResponse {
    /// Engine identification (name or author)
    #[serde(rename = "id")]
    Id(IdInfo),

    /// Engine is ready in UCI mode
    #[serde(rename = "uciok")]
    UciOk,

    /// Engine is ready after receiving "isready"
    #[serde(rename = "readyok")]
    ReadyOk,

    /// Engine's best move and optional ponder move
    #[serde(rename = "bestmove")]
    BestMove {
        best_move: String,
        ponder: Option<String>,
    },

    /// Copy protection status
    #[serde(rename = "copyprotection")]
    CopyProtection(ProtectionStatus),

    /// Registration status
    #[serde(rename = "registration")]
    Registration(RegistrationStatus),

    /// Search information
    #[serde(rename = "info")]
    Info(InfoParams),

    /// Option definition
    #[serde(rename = "option")]
    Option(OptionDefinition),
}

/// Engine identification information
#[derive(Serialize)]
#[serde(tag = "id_type", content = "value")]
pub enum IdInfo {
    /// Engine name
    #[serde(rename = "name")]
    Name(String),

    /// Engine author
    #[serde(rename = "author")]
    Author(String),
}

/// Protection status for copyprotection and registration
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProtectionStatus {
    /// Checking status
    Checking,

    /// Status is OK
    Ok,

    /// Status has an error
    Error,
}

/// Alias for registration status
pub type RegistrationStatus = ProtectionStatus;

/// Parameters for the "info" response
#[derive(Serialize, Default)]
pub struct InfoParams {
    /// Search depth in plies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,

    /// Selective search depth in plies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seldepth: Option<u32>,

    /// Search time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,

    /// Nodes searched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<u64>,

    /// Principal variation (best line)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<Vec<String>>,

    /// Multi-PV number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipv: Option<u32>,

    /// Score information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<Score>,

    /// Current move being searched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currmove: Option<String>,

    /// Current move number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currmovenumber: Option<u32>,

    /// Hash table usage in permill
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashfull: Option<u32>,

    /// Nodes per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nps: Option<u64>,

    /// Tablebase hits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbhits: Option<u64>,

    /// Shredder endgame database hits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sbhits: Option<u64>,

    /// CPU usage in permill
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuload: Option<u32>,

    /// Arbitrary string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,

    /// Refutation line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refutation: Option<Vec<String>>,

    /// Current line being calculated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currline: Option<(Option<u32>, Vec<String>)>,
}

/// Score information in the "info" response
#[derive(Serialize)]
#[serde(untagged)]
pub enum Score {
    /// Score in centipawns
    Centipawns {
        value: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        bound: Option<Bound>,
    },

    /// Mate in N moves
    Mate(i32),
}

/// Score bound
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bound {
    /// Score is just a lower bound
    Lower,

    /// Score is just an upper bound
    Upper,
}

/// Option definition sent by the engine
#[derive(Serialize)]
pub struct OptionDefinition {
    /// Option name
    pub name: String,

    /// Option type
    #[serde(rename = "type")]
    pub option_type: OptionType,

    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Minimum value for "spin" options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,

    /// Maximum value for "spin" options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,

    /// Predefined values for "combo" options
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub var: Vec<String>,
}

/// Types of options
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
    /// Checkbox (boolean)
    Check,

    /// Spin wheel (integer)
    Spin,

    /// Combo box (predefined strings)
    Combo,

    /// Button (no value)
    Button,

    /// Text field (string)
    String,
}

/// Error type for parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Failed to parse the response
    ParseFailure { input: String, message: String },

    /// Invalid value for a parameter
    InvalidValue { param: String, value: String },

    /// Missing value for a parameter
    MissingValue { param: String },

    /// Unknown response type
    UnknownResponseType { token: String },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFailure { input, message } => write!(f, "Parse failure: {}", message),
            Self::InvalidValue { param, value } => {
                write!(f, "Invalid value for parameter: {}", param)
            }
            Self::MissingValue { param } => write!(f, "Missing value for parameter: {}", param),
            Self::UnknownResponseType { token } => write!(f, "Unknown response type: {}", token),
        }
    }
}

/// Top-level parser for engine responses
fn engine_response_parser() -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    let simple_parsers = choice((
        id_parser(),
        uciok_parser(),
        readyok_parser(),
        bestmove_parser(),
        copyprotection_parser(),
        registration_parser(),
    ));

    let info_parser = info_token_parser().try_map(|s, span| {
        parse_info_params(s).map_err(|e| Simple::custom(span, format!("{:?}", e)))
    });

    let option_parser = option_token_parser().try_map(|s, span| {
        parse_option_params(s).map_err(|e| Simple::custom(span, format!("{:?}", e)))
    });

    simple_parsers
        .or(info_parser)
        .or(option_parser)
        .then_ignore(end())
        .labelled("engine response")
}

/// Public function to parse an engine response
pub fn parse_engine_response(line: &str) -> Result<EngineResponse, ParseError> {
    engine_response_parser()
        .parse(line)
        .map_err(|e| ParseError::ParseFailure {
            input: line.to_string(),
            message: e
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        })
}
