use crate::engine::events::EngineStateInfoEvent;
use crate::engine::state::engine_state::{EngineReadyState, EngineStateInfo};
use crate::engine::utils::EngineError;
use ok_parse::uci::{parse_engine_response, EngineResponse};

use super::{EngineCommand, ParserOutput, ProtocolComposer, ProtocolParser};

/// ProtocolParser for UCI protocol
pub struct UciProtocolParser;

impl ProtocolParser for UciProtocolParser {
    type State = EngineStateInfo;
    type Output = ParserOutput<EngineStateInfo>;

    fn parse_line(&self, line: &str) -> Result<ParserOutput<EngineStateInfo>, EngineError> {
        match parse_engine_response(line) {
            Ok(tokens) => match tokens {
                EngineResponse::UciOk => Ok(ParserOutput::StateUpdate(
                    EngineStateInfoEvent::ReadyStateChanged(EngineReadyState::Initialized),
                )),
                EngineResponse::ReadyOk => Ok(ParserOutput::StateUpdate(
                    EngineStateInfoEvent::ReadyStateChanged(EngineReadyState::Ready),
                )),
                EngineResponse::Id(id) => Ok(ParserOutput::StateUpdate(
                    EngineStateInfoEvent::InfoUpdate(id),
                )),
                EngineResponse::Option(option) => Ok(ParserOutput::StateUpdate(
                    EngineStateInfoEvent::CapabilityAdded(option.name.clone(), option),
                )),
                EngineResponse::Info(info) => Ok(ParserOutput::StateUpdate(
                    EngineStateInfoEvent::AnalysisUpdate(info),
                )),
                EngineResponse::BestMove { best_move, ponder } => Ok(ParserOutput::StateUpdate(
                    EngineStateInfoEvent::BestMove(best_move, ponder),
                )),
                _ => Err(EngineError::ProtocolFailedToParseLine(line.to_string())),
            },
            Err(_e) => Err(EngineError::ProtocolFailedToParseLine(line.to_string())),
        }
    }

    fn protocol_name(&self) -> &'static str {
        "UCI"
    }
}

/// Implementation for the Universal Chess Interface (UCI) protocol
pub struct UciProtocolComposer;

impl ProtocolComposer for UciProtocolComposer {
    fn compose(&self, command: EngineCommand) -> Result<String, EngineError> {
        match command {
            EngineCommand::Raw(cmd) => Ok(cmd),
            EngineCommand::IsReady => Ok("isready".to_string()),
            EngineCommand::NewGame => Ok("ucinewgame".to_string()),
            EngineCommand::SetPosition { fen, moves } => {
                let mut cmd = String::from("position");

                if let Some(fen_str) = fen {
                    cmd.push_str(&format!(" fen {}", fen_str));
                } else {
                    cmd.push_str(" startpos");
                }

                if let Some(move_list) = moves {
                    if !move_list.is_empty() {
                        cmd.push_str(" moves");
                        for m in move_list {
                            cmd.push_str(&format!(" {}", m));
                        }
                    }
                }

                Ok(cmd)
            }
            EngineCommand::StartAnalysis {
                depth,
                movetime,
                nodes,
                multipv,
                searchmoves,
            } => {
                let mut cmd = String::from("go");

                if let Some(d) = depth {
                    cmd.push_str(&format!(" depth {}", d));
                }

                if let Some(mt) = movetime {
                    cmd.push_str(&format!(" movetime {}", mt));
                }

                if let Some(n) = nodes {
                    cmd.push_str(&format!(" nodes {}", n));
                }

                if let Some(mpv) = multipv {
                    cmd.push_str(&format!(" multipv {}", mpv));
                }

                if let Some(sm) = searchmoves {
                    if !sm.is_empty() {
                        cmd.push_str(" searchmoves");
                        for m in sm {
                            cmd.push_str(&format!(" {}", m));
                        }
                    }
                }

                Ok(cmd)
            }
            EngineCommand::StopAnalysis => Ok("stop".to_string()),
            EngineCommand::SetOption { name, value } => {
                Ok(format!("setoption name {} value {}", name, value))
            }
            EngineCommand::Quit => Ok("quit".to_string()),
        }
    }

    fn protocol_name(&self) -> &str {
        "UCI"
    }

    fn supports_feature(&self, feature: &str) -> bool {
        match feature {
            "multipv" | "searchmoves" | "depth" | "movetime" | "nodes" => true,
            _ => false,
        }
    }

    fn initial_command(&self) -> Result<EngineCommand, EngineError> {
        Ok(EngineCommand::Raw("uci".to_string()))
    }
}
