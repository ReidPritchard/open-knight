use std::sync::Arc;

use tokio::io::AsyncWriteExt;
use tokio::process::ChildStdin;
use tokio::sync::RwLock;

use crate::engine::events::EngineStateInfoEvent;
use crate::engine::protocol::{EngineCommand, OptionValue, ProtocolComposer};
use crate::engine::state::engine_state::EngineReadyState;
use crate::engine::state::EngineState;
use crate::engine::utils::EngineError;

/// Handles sending commands to the engine
///
/// Used to send commands to stdin using the protocol composer.
/// It abstracts the protocol details from the user and provides
/// a clean interface for sending various types of commands.
pub struct InputHandler<S: EngineState> {
    input_stream: ChildStdin,
    protocol_composer: Box<dyn ProtocolComposer>,
    state: Arc<RwLock<S>>,
}

impl<S: EngineState> InputHandler<S> {
    pub fn new(
        input_stream: ChildStdin,
        protocol_composer: Box<dyn ProtocolComposer>,
        state: Arc<RwLock<S>>,
    ) -> Self {
        Self {
            input_stream,
            protocol_composer,
            state,
        }
    }

    /// Send a raw string to the engine
    ///
    /// This is an internal function for sending the formatted command
    async fn send_raw(&mut self, cmd: &str) -> Result<(), EngineError> {
        // No formatting, just send the raw string
        if let Err(e) = self.input_stream.write_all(cmd.as_bytes()).await {
            return Err(EngineError::IoFailedToWriteLine(e.to_string()));
        }

        // Add a newline
        if let Err(e) = self.input_stream.write_all(b"\n").await {
            return Err(EngineError::IoFailedToWriteLine(e.to_string()));
        }

        // Flush the buffer
        if let Err(e) = self.input_stream.flush().await {
            return Err(EngineError::IoFailedToFlush(e.to_string()));
        }

        Ok(())
    }

    /// Send a command to the engine
    pub async fn send_command(&mut self, command: EngineCommand) -> Result<(), EngineError> {
        let formatted = self.protocol_composer.compose(command)?;
        self.send_raw(&formatted).await
    }

    /// Send a raw command string to the engine
    ///
    /// This will still be formatted according to the protocol
    pub async fn send_raw_command(&mut self, command: &str) -> Result<(), EngineError> {
        let formatted = self
            .protocol_composer
            .compose(EngineCommand::Raw(command.to_string()))?;
        self.send_raw(&formatted).await
    }

    /// Check if the engine is ready
    pub async fn is_ready(&mut self) -> Result<(), EngineError> {
        self.send_command(EngineCommand::IsReady).await
    }

    /// Start a new game
    pub async fn new_game(&mut self) -> Result<(), EngineError> {
        self.send_command(EngineCommand::NewGame).await
    }

    /// Set the engine's position
    pub async fn set_position(
        &mut self,
        fen: Option<&str>,
        moves: Option<&[&str]>,
    ) -> Result<(), EngineError>
    where
        S: EngineState<Update = EngineStateInfoEvent>,
    {
        let fen_owned = fen.map(String::from);
        let moves_owned = moves.map(|m| m.iter().map(|&s| s.to_string()).collect());

        let cmd_res = self
            .send_command(EngineCommand::SetPosition {
                fen: fen_owned.clone(),
                moves: moves_owned,
            })
            .await;

        cmd_res?;

        // Update the state
        let state_update =
            <S as EngineState>::Update::CurrentPositionChanged(fen_owned.unwrap_or_default());

        self.state.write().await.apply_update(state_update)?;
        Ok(())
    }

    /// Start analysis with optional depth and time constraints
    pub async fn start_analysis(
        &mut self,
        depth: Option<u32>,
        movetime: Option<u32>,
    ) -> Result<(), EngineError>
    where
        S: EngineState<Update = EngineStateInfoEvent>,
    {
        let cmd_res = self
            .send_command(EngineCommand::StartAnalysis {
                depth,
                movetime,
                nodes: None,
                multipv: None,
                searchmoves: None,
            })
            .await;

        match cmd_res {
            Ok(_) => {
                // Update the state
                let state_update =
                    <S as EngineState>::Update::ReadyStateChanged(EngineReadyState::Analyzing);
                self.state.write().await.apply_update(state_update)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Start advanced analysis with additional parameters
    pub async fn start_advanced_analysis(
        &mut self,
        depth: Option<u32>,
        movetime: Option<u32>,
        nodes: Option<u64>,
        multipv: Option<u32>,
        searchmoves: Option<&[&str]>,
    ) -> Result<(), EngineError>
    where
        S: EngineState<Update = EngineStateInfoEvent>,
    {
        let searchmoves_owned = searchmoves.map(|m| m.iter().map(|&s| s.to_string()).collect());

        let cmd_res = self
            .send_command(EngineCommand::StartAnalysis {
                depth,
                movetime,
                nodes,
                multipv,
                searchmoves: searchmoves_owned,
            })
            .await;

        cmd_res?;

        Ok(())
    }

    /// Stop ongoing analysis
    pub async fn stop_analysis(&mut self) -> Result<(), EngineError> {
        self.send_command(EngineCommand::StopAnalysis).await
    }

    /// Set an engine option
    pub async fn set_option(&mut self, name: &str, value: OptionValue) -> Result<(), EngineError> {
        self.send_command(EngineCommand::SetOption {
            name: name.to_string(),
            value,
        })
        .await
    }

    /// Quit the engine
    pub async fn quit(&mut self) -> Result<(), EngineError> {
        self.send_command(EngineCommand::Quit).await
    }

    /// Get the protocol name being used
    pub fn protocol_name(&self) -> &str {
        self.protocol_composer.protocol_name()
    }

    /// Check if a specific feature is supported by the current protocol
    pub fn supports_feature(&self, feature: &str) -> bool {
        self.protocol_composer.supports_feature(feature)
    }
}
