use std::collections::HashMap;

use crate::engine::events::EngineStateInfoEvent;
use crate::parse::uci::{IdInfo, InfoParams, OptionDefinition, ProtectionStatus};
use serde::Serialize;

/// Engine readiness (initialized, running, analyzing)
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum EngineReadyState {
    /// The engine is not running
    NotRunning,
    /// The engine is running, but not yet initialized
    Starting,
    /// The engine has been initialized
    Initialized,
    /// The engine is initialized and ready for commands
    Ready,
    /// The engine has been sent a command but has not completed it
    ///
    /// This is for commands like `ucinewgame` which can take a while to complete
    /// only an `isready` command should be sent to check if the engine is ready in this state
    Busy,
    /// The engine is analyzing a position
    Analyzing,
    // TODO: Add more states as needed
}

/// Generic engine information
#[derive(Debug, Serialize, Default, Clone)]
pub struct EngineMetadata {
    /// The engine's version
    pub version: Option<String>,
    /// The engine's name
    pub name: Option<String>,
    /// The engine's author
    pub author: Option<String>,
    /// The engine's copy protection status
    pub copy_protection: Option<ProtectionStatus>,
}

/// General/generic internal engine state
#[derive(Debug, Serialize, Clone)]
pub struct EngineStateInfo {
    /// The engine's info
    pub info: EngineMetadata,
    /// The engine's readiness state
    pub ready_state: EngineReadyState,
    /// The engine's capabilities
    pub capabilities: HashMap<String, OptionDefinition>,
    /// The current position
    pub current_position: Option<String>,
    /// Ongoing analysis
    pub analysis: Option<Analysis>,
    /// The engine's best move in the current analysis
    pub best_move: Option<(String, Option<String>)>,
}

/// Ongoing analysis
#[derive(Debug, Serialize, Clone)]
pub struct Analysis {
    /// A vector of the recieved analysis updates
    /// leading to the "bestmove" and reset on subseqent calls to `start_analysis`
    pub updates: Vec<InfoParams>,
}

/// Default engine state
impl Default for EngineStateInfo {
    fn default() -> Self {
        Self {
            info: EngineMetadata::default(),
            ready_state: EngineReadyState::NotRunning,
            capabilities: HashMap::new(),
            current_position: None,
            analysis: None,
            best_move: None,
        }
    }
}

impl EngineStateInfo {
    /// Set the engine's readiness state
    pub fn set_ready_state(&mut self, ready_state: EngineReadyState) {
        self.ready_state = ready_state;
    }

    /// Add a capability to the engine
    pub fn add_capability(&mut self, capability: OptionDefinition) {
        self.capabilities
            .insert(capability.name.clone(), capability);
    }

    /// Set the current position
    pub fn set_current_position(&mut self, position: String) {
        self.current_position = Some(position);
    }

    /// Set the ongoing analysis
    pub fn set_analysis(&mut self, analysis: Analysis) {
        self.analysis = Some(analysis);
    }

    /// Get the engine's info as a JSON string
    pub fn json_info(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Get the engine's debug info
    pub fn debug_info(&self) -> String {
        let mut info = String::new();
        info.push_str(&format!("Ready State: {:?}\n", self.ready_state));
        info.push_str(&format!("Current Position: {:?}\n", self.current_position));
        if let Some(analysis) = &self.analysis {
            info.push_str(&format!("Analysis Updates:\n"));
            for update in &analysis.updates {
                info.push_str(&format!("\t---\n"));
                info.push_str(&format!("{}", update));
            }
        }
        if let Some(best_move) = &self.best_move {
            let (best_move, ponder) = best_move;
            info.push_str(&format!("Best Move:\n\t{}\n", best_move));
            if let Some(ponder) = ponder {
                info.push_str(&format!("\tPonder: {}\n", ponder));
            }
        }

        info.push_str(&format!("Capabilities:\n"));
        for (name, cap) in &self.capabilities {
            info.push_str(&format!("    {} ({:?})\n", name, cap.option_type));
            if let Some(default) = &cap.default {
                info.push_str(&format!("        default: {}\n", default));
            }
            if cap.min.is_some() || cap.max.is_some() {
                let min = cap.min.unwrap_or(0);
                let max = cap.max.unwrap_or(0);
                info.push_str(&format!("        range: {:?}..{:?}\n", min, max));
            }
            if !cap.var.is_empty() {
                info.push_str(&format!("        options: {:?}\n", cap.var));
            }
        }
        info
    }
}

/// Engine state update implementation
impl super::EngineState for EngineStateInfo {
    type Update = EngineStateInfoEvent;
    type Event = EngineStateInfoEvent;

    fn apply_update(&mut self, update: Self::Update) -> Result<Self::Event, crate::EngineError> {
        match update {
            EngineStateInfoEvent::InfoUpdate(info) => {
                let id_info = info.clone();
                match info {
                    IdInfo::Name(name) => {
                        self.info.name = Some(name);
                    }
                    IdInfo::Author(author) => {
                        self.info.author = Some(author);
                    }
                }
                Ok(EngineStateInfoEvent::InfoUpdate(id_info))
            }
            EngineStateInfoEvent::CapabilityAdded(name, cap) => {
                self.capabilities.insert(name.clone(), cap.clone());
                Ok(EngineStateInfoEvent::CapabilityAdded(name, cap))
            }
            EngineStateInfoEvent::AnalysisUpdate(analysis) => {
                // If the analysis is not set, create a new one
                if self.analysis.is_none() {
                    self.analysis = Some(Analysis {
                        updates: Vec::new(),
                    });
                }

                // Add the new update to the analysis
                self.analysis
                    .as_mut()
                    .unwrap()
                    .updates
                    .push(analysis.clone());
                Ok(EngineStateInfoEvent::AnalysisUpdate(analysis))
            }
            EngineStateInfoEvent::BestMove(best_move, ponder) => {
                self.best_move = Some((best_move.clone(), ponder.clone()));

                // Analysis is complete, update the ready state
                self.ready_state = EngineReadyState::Ready;

                Ok(EngineStateInfoEvent::BestMove(best_move, ponder))
            }
            EngineStateInfoEvent::ReadyStateChanged(ready_state) => {
                self.ready_state = ready_state.clone();
                Ok(EngineStateInfoEvent::ReadyStateChanged(ready_state))
            }
            EngineStateInfoEvent::CurrentPositionChanged(position) => {
                self.current_position = Some(position.clone());

                // Reset the analysis
                self.analysis = None;

                Ok(EngineStateInfoEvent::CurrentPositionChanged(position))
            }
            EngineStateInfoEvent::LifecycleEvent(event) => {
                Ok(EngineStateInfoEvent::LifecycleEvent(event))
            }
            EngineStateInfoEvent::Error(error) => Err(error),
        }
    }
}
