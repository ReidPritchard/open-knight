use tokio::process::Command;

use super::process::EngineProcess;
use super::state::EngineState;

/// Builder for an engine process
///
/// Accepts all the necessary information to build an EngineProcess
/// Calling `build` will return the EngineProcess
pub struct EngineProcessBuilder<S: EngineState> {
    /// The command to launch the engine
    command: Option<Command>,
    /// The state of the engine
    state: Option<S>,
}

impl<S: EngineState> Default for EngineProcessBuilder<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: EngineState> EngineProcessBuilder<S> {
    pub fn new() -> Self {
        Self {
            command: None,
            state: None,
        }
    }

    /// Set the command to launch the engine
    pub fn command(
        mut self,
        command: Command,
    ) -> Self {
        self.command = Some(command);
        self
    }

    /// Set the initial state of the engine
    pub fn state(
        mut self,
        state: S,
    ) -> Self {
        self.state = Some(state);
        self
    }

    /// Build the engine process
    pub fn build(self) -> EngineProcess<S> {
        // Make sure all required fields are set
        if self.command.is_none() {
            panic!("Command is required");
        }

        if self.state.is_none() {
            panic!("State is required");
        }

        // Create the engine process
        EngineProcess::new(self.command.unwrap(), self.state.unwrap())
    }
}
