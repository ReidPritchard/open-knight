use std::process::Stdio;
use std::sync::Arc;

use log::{debug, error};
use tokio::io::BufReader;
use tokio::process::{Child, Command};
use tokio::signal::unix::Signal;
use tokio::sync::{broadcast, RwLock, RwLockReadGuard};

use super::builder::EngineProcessBuilder;
use super::events::{EngineStateInfoEvent, EventBus};
use super::io_handler::input_handler::InputHandler;
use super::io_handler::output_handler::OutputHandler;
use super::protocol::{ProtocolComposerType, ProtocolParserType};
use super::state::engine_state::EngineReadyState;
use super::state::EngineState;
use super::utils::EngineError;

/// The main interface for engine management
///
/// This struct is responsible for managing the lifecycle of an engine process
/// and handling all the events and state updates for the engine.
///
/// The manager is designed for async operations and uses tokio for concurrency.
/// The functionality is split into several components:
/// - IO handling (stdin/stdout/stderr)
///     - Reads and writes to the engine process
/// - State handling (engine state updates)
///     - Applies updates to the engine state
/// - Event handling (lifecycle, state changes)
///     - Emits events to the event bus for subscribers to handle
/// - Protocol handling (UCI, XBoard, etc.)
///     - Translates raw IO into engine events
///
/// The goal is to provide a simple interface for managing any engine process
/// while optimizing for concurrency and event-driven architectures.
pub struct EngineProcess<S: EngineState>
where
    <S as EngineState>::Event: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    /// The command used to launch the engine
    command: Command,
    /// The child process
    child: Option<Child>,
    /// The engine state
    state: Arc<RwLock<S>>,
    /// The input handler
    input_handler: Option<InputHandler<S>>,
    /// The output handler
    output_handler: Option<OutputHandler<S, <S as EngineState>::Event>>,
    /// The shutdown signal
    shutdown_tx: broadcast::Sender<()>,
}

/// Process configuration and lifecycle management
impl<S: EngineState> EngineProcess<S>
where
    <S as EngineState>::Event: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    /// Create a new engine process
    pub fn new(command: Command, state: S) -> Self {
        let (shutdown_tx, _) = broadcast::channel(16);
        let state = Arc::new(RwLock::new(state));

        Self {
            command,
            child: None,
            input_handler: None,
            output_handler: None,
            state,
            shutdown_tx,
        }
    }

    /// Start building a new engine process
    pub fn builder() -> EngineProcessBuilder<S> {
        EngineProcessBuilder::new()
    }

    /// Spawn a new engine process
    pub async fn spawn(
        &mut self,
        protocol_parser: ProtocolParserType<S>,
        protocol_composer: ProtocolComposerType,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut child = self
            .command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
        // let stderr = child.stderr.take().ok_or("Failed to open stderr")?;

        self.child = Some(child);

        // Setup the output handler
        let mut output_handler = OutputHandler::<S, <S as EngineState>::Event>::new(
            BufReader::new(stdout),
            protocol_parser,
            self.state.clone(),
            self.shutdown_tx.subscribe(),
        );
        // Start the output handler
        match output_handler.start().await {
            Ok(()) => {
                debug!("Output handler started");
                self.output_handler = Some(output_handler);
            }
            Err(e) => {
                error!("Failed to start output handler: {:?}", e);
                // TODO: Handle the error, probably just kill the process and return this error
            }
        }

        // Setup the input handler

        // Get the initial command before moving the protocol to the input handler
        let initial_command = protocol_composer.initial_command();

        let mut input_handler = InputHandler::new(stdin, protocol_composer, self.state.clone());

        // Send the initial command to the engine
        if let Ok(initial_command) = initial_command {
            match input_handler.send_command(initial_command).await {
                Ok(_) => {
                    self.input_handler = Some(input_handler);
                }
                Err(e) => {
                    // TODO: Handle the error
                    error!("Failed to send initial command: {:?}", e);
                }
            }
        }

        Ok(())
    }

    /// Stop the engine process
    pub async fn kill(&mut self, _signal: Option<Signal>) -> Result<(), EngineError> {
        // Graceful or forced termination
        if let Some(child) = &mut self.child {
            if let Err(e) = child.kill().await {
                return Err(EngineError::ProcessFailedToKill(e.to_string()));
            }
        }
        Ok(())
    }
}

/// Methods for state access
impl<S: EngineState> EngineProcess<S>
where
    <S as EngineState>::Event: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    pub async fn get_state(&self) -> S {
        // Create a snapshot of the state
        let state = self.state.read().await;
        state.clone()
    }

    pub async fn query_state<F, R>(&self, query: F) -> R
    where
        F: FnOnce(RwLockReadGuard<S>) -> R,
    {
        let state = self.state.read().await;
        query(state)
    }
}

/// Getters for engine process properties
impl<S: EngineState> EngineProcess<S>
where
    <S as EngineState>::Event: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    /// Get the event bus
    pub fn event_bus(&self) -> Result<&EventBus, EngineError> {
        self.output_handler
            .as_ref()
            .map(|handler| handler.event_bus().as_ref())
            .ok_or_else(|| EngineError::InvalidState("Output handler not initialized".to_string()))
    }

    /// Get the input handler
    pub fn input_handler(&mut self) -> Result<&mut InputHandler<S>, EngineError> {
        self.input_handler
            .as_mut()
            .ok_or_else(|| EngineError::InvalidState("Input handler not initialized".to_string()))
    }
}

/// Convenient methods for UX
impl<S: EngineState> EngineProcess<S>
where
    <S as EngineState>::Event: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    /// Method to monitor engine events
    ///
    /// The callback should return:
    /// - `true` to continue monitoring events
    /// - `false` to stop monitoring and return successfully
    ///
    /// Returns an error if the event bus is not available or if an error event is received
    pub async fn monitor_events<Callback>(&self, mut callback: Callback) -> Result<(), EngineError>
    where
        Callback: FnMut(EngineStateInfoEvent) -> bool,
    {
        let event_bus = self.event_bus()?;
        let mut event_stream = event_bus.subscribe::<EngineStateInfoEvent>();

        loop {
            let event = event_stream.recv().await;
            if let Some(event) = event {
                match event {
                    EngineStateInfoEvent::Error(error) => return Err(error),
                    _ => {
                        if !callback(event) {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    /// Wait until the engine is in the specified ready state
    pub async fn wait_until_ready(
        &self,
        target_state: EngineReadyState,
    ) -> Result<(), EngineError> {
        let event_bus = self.event_bus()?;
        let mut event_stream = event_bus.subscribe::<EngineStateInfoEvent>();

        loop {
            let event = event_stream.recv().await;
            if let Some(event) = event {
                match event {
                    EngineStateInfoEvent::ReadyStateChanged(state) => {
                        if state == target_state {
                            return Ok(());
                        }
                    }
                    EngineStateInfoEvent::Error(error) => {
                        return Err(error);
                    }
                    _ => {}
                }
            }
        }
    }
}
