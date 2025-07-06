use chumsky::prelude::*;
use complex::{
    info_token_parser, option_token_parser, parse_info_params,
    parse_option_params,
};
use serde::Serialize;
use simple::{
    bestmove_parser, copyprotection_parser, id_parser, readyok_parser,
    registration_parser, uciok_parser,
};

use crate::DEBUG;

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
#[derive(Serialize, Clone, Debug)]
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
#[derive(Serialize, Clone, Debug)]
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
#[derive(Serialize, Default, Clone, Debug)]
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

/// Pretty print the InfoParams
/// print each field on a new line with indent
/// skip empty fields
impl std::fmt::Display for InfoParams {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut fields = vec![];
        if let Some(depth) = self.depth {
            fields.push(format!("Depth: {}", depth));
        }
        if let Some(seldepth) = self.seldepth {
            fields.push(format!("Seldepth: {}", seldepth));
        }
        if let Some(time) = self.time {
            fields.push(format!("Time: {}", time));
        }
        if let Some(nodes) = self.nodes {
            fields.push(format!("Nodes: {}", nodes));
        }
        if let Some(nps) = self.nps {
            fields.push(format!("NPS: {}", nps));
        }
        if let Some(hashfull) = self.hashfull {
            fields.push(format!("Hashfull: {}", hashfull));
        }
        if let Some(tbhits) = self.tbhits {
            fields.push(format!("TBhits: {}", tbhits));
        }
        if let Some(sbhits) = self.sbhits {
            fields.push(format!("SBhits: {}", sbhits));
        }
        if let Some(cpuload) = self.cpuload {
            fields.push(format!("CPULoad: {}", cpuload));
        }
        if let Some(string) = &self.string {
            fields.push(format!("String: {}", string));
        }
        if let Some(refutation) = &self.refutation {
            fields.push(format!("Refutation: {}", refutation.join(", ")));
        }
        if let Some(currline) = &self.currline {
            fields.push(format!("Currline: {}", currline.1.join(", ")));
        }
        if let Some(score) = &self.score {
            fields.push(format!("Score: {:?}", score));
        }
        for field in fields {
            write!(f, "\t{}\n", field)?;
        }
        Ok(())
    }
}

/// Score information in the "info" response
#[derive(Serialize, Clone, Debug)]
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
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Bound {
    /// Score is just a lower bound
    Lower,

    /// Score is just an upper bound
    Upper,
}

/// Option definition sent by the engine
#[derive(Serialize, Clone, Debug)]
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
#[derive(Serialize, Clone, Debug)]
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
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("UCI parse error: {0}")]
pub enum UciParseError {
    /// Failed to parse the response
    #[error("Parse failure: \n\tInput: '{input}'\n\tMessage: {message}")]
    ParseFailure { input: String, message: String },

    /// Invalid value for a parameter
    #[error("Invalid value for parameter: \n\tParameter: '{param}'\n\tValue: '{value}'")]
    InvalidValue { param: String, value: String },

    /// Missing value for a parameter
    #[error("Missing value for parameter: \n\tParameter: '{param}'")]
    MissingValue { param: String },

    /// Unknown response type
    #[error("Unknown response type: \n\tToken: '{token}'")]
    UnknownResponseType { token: String },

    /// Unknown protocol
    #[error("Unknown protocol: \n\tName: '{name}'")]
    UnknownProtocol { name: String },
}

/// Top-level parser for engine responses
fn engine_response_parser(
) -> impl Parser<char, EngineResponse, Error = Simple<char>> {
    let simple_parsers = choice((
        id_parser(),
        uciok_parser(),
        readyok_parser(),
        bestmove_parser(),
        copyprotection_parser(),
        registration_parser(),
    ));

    let info_parser = info_token_parser().try_map(|s, span| {
        parse_info_params(s)
            .map_err(|e| Simple::custom(span, format!("{:?}", e)))
    });

    let option_parser = option_token_parser().try_map(|s, span| {
        parse_option_params(s)
            .map_err(|e| Simple::custom(span, format!("{:?}", e)))
    });

    simple_parsers
        .or(info_parser)
        .or(option_parser)
        .then_ignore(end())
        .labelled("engine response")
}

/// Public function to parse an engine response
pub fn parse_engine_response(
    line: &str
) -> Result<EngineResponse, UciParseError> {
    if DEBUG {
        let (response, errors) =
            engine_response_parser().parse_recovery_verbose(line);
        response.ok_or_else(|| UciParseError::ParseFailure {
            input: line.to_string(),
            message: errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        })
    } else {
        engine_response_parser().parse(line).map_err(|e| {
            UciParseError::ParseFailure {
                input: line.to_string(),
                message: e
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            }
        })
    }
}
