use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStdout;
use tokio::select;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::task::JoinHandle;

use crate::engine::events::{EngineStateInfoEvent, EventBus};
use crate::engine::protocol::{ParserOutput, ProtocolParser};
use crate::engine::state::EngineState;
use crate::engine::utils::EngineError;

/// A struct for handling the output of an engine
///
/// The main purpose of this struct is to manage async communication between the engine
/// and the main thread to avoid blocking the main thread.
/// This is done by spawning a separate task to first parse the output (using a `ProtocolParser`),
/// then process the parsed output (likely applying updates to the engine state),
/// and finally sending the events to the event bus to be handled by any subscribers/listeners.
pub struct OutputHandler<S: EngineState, E = <S as EngineState>::Event>
where
    E: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    reader: Option<BufReader<ChildStdout>>,
    parser: Option<Box<dyn ProtocolParser<State = S, Output = ParserOutput<S>>>>,
    state: Arc<RwLock<S>>,
    event_bus: Arc<EventBus>,
    shutdown_rx: broadcast::Receiver<()>,
    task_handle: Option<JoinHandle<()>>,
    _phantom: std::marker::PhantomData<E>,
}

impl<S: EngineState, E> OutputHandler<S, E>
where
    E: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    pub fn new(
        reader: BufReader<ChildStdout>,
        parser: Box<dyn ProtocolParser<State = S, Output = ParserOutput<S>>>,
        state: Arc<RwLock<S>>,
        shutdown_rx: broadcast::Receiver<()>,
    ) -> Self {
        let event_bus = Arc::new(EventBus::new());

        Self {
            reader: Some(reader),
            parser: Some(parser),
            state,
            event_bus,
            shutdown_rx,
            task_handle: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Starts the output handler in a separate task/thread
    ///
    /// This spawns an asynchronous task that processes the output
    /// without blocking the main thread.
    pub async fn start(&mut self) -> Result<(), EngineError> {
        // Ensure we haven't already started
        if self.task_handle.is_some() {
            return Err(EngineError::OutputHandlerAlreadyStarted(
                "Output handler already started".into(),
            ));
        }

        // Take ownership of reader and parser from self
        let reader = self
            .reader
            .take()
            .ok_or_else(|| EngineError::InvalidState("Reader already taken".into()))?;

        let parser = self
            .parser
            .take()
            .ok_or_else(|| EngineError::InvalidState("Parser already taken".into()))?;

        // Clone shared components
        let state = self.state.clone();

        // Create a channel for sending events back to the main event bus
        let (event_sender, mut event_receiver) = mpsc::channel::<S::Event>(100);

        // Take ownership of the shutdown receiver
        let shutdown_rx = std::mem::replace(
            &mut self.shutdown_rx,
            broadcast::channel(1).1, // Replace with a dummy receiver
        );

        // Spawn a task to forward events from the channel to the event bus
        let event_bus = Arc::clone(&self.event_bus);
        let _forward_handle = tokio::spawn(async move {
            while let Some(event) = event_receiver.recv().await {
                event_bus.publish(event);
            }
        });

        // Spawn the main processing task
        let handle = tokio::spawn(async move {
            process_output::<S, S::Event>(reader, parser, state, event_sender, shutdown_rx).await;
        });

        // Store the task handle
        self.task_handle = Some(handle);

        Ok(())
    }

    /// Returns a reference to the event bus
    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }

    /// Waits for the output handler task to complete
    pub async fn join(&mut self) -> Result<(), EngineError> {
        if let Some(handle) = self.task_handle.take() {
            handle.await.map_err(|e| {
                EngineError::IoFailedToJoin(format!("Failed to join output handler task: {}", e))
            })?;
        }
        Ok(())
    }
}

/// Process output in a separate function that takes ownership of its parameters
async fn process_output<State: EngineState<Event = E>, E>(
    mut reader: BufReader<ChildStdout>,
    parser: Box<dyn ProtocolParser<State = State, Output = ParserOutput<State>>>,
    state: Arc<RwLock<State>>,
    event_sender: mpsc::Sender<E>,
    mut shutdown_rx: broadcast::Receiver<()>,
) where
    E: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    let mut buffer = String::new();

    'reader: loop {
        select! {
            result = reader.read_line(&mut buffer) => {
                match result {
                    Ok(0) => break 'reader, // EOF
                    Ok(_) => {
                        handle_line::<State, E>(&parser, &state, &event_sender, &buffer).await;
                    }
                    Err(e) => {
                        println!("Error reading line: {:?}", e);
                        // handle_error(&event_sender, e).await;
                        // For generic E, error handling must be done by the caller
                    }
                }
            }
            _ = shutdown_rx.recv() => break 'reader,
        }
        buffer.clear();
    }
}

async fn handle_line<State: EngineState<Event = E>, E>(
    parser: &Box<dyn ProtocolParser<State = State, Output = ParserOutput<State>>>,
    state: &Arc<RwLock<State>>,
    event_sender: &mpsc::Sender<E>,
    line: &str,
) where
    E: Send + Sync + Clone + std::fmt::Debug + 'static,
{
    // Parse the line with a ProtocolParser and return the ParserOutput
    match parser.parse_line(line) {
        Ok(ParserOutput::StateUpdate(update)) => {
            // Apply the update to the state
            let event = state.write().await.apply_update(update);
            match event {
                Ok(event) => {
                    // Send the event to the event bus
                    let _ = event_sender.send(event).await;
                }
                Err(_) => {
                    // Error handling for generic E must be done by the caller
                    println!("Error applying update: {:?}", event);
                    return;
                }
            };
        }
        Ok(ParserOutput::LifecycleEvent(_event)) => {
            // For generic E, lifecycle event handling must be done by the caller
        }
        Ok(ParserOutput::NoUpdate) => {}
        Err(_) => {
            // Error handling for generic E must be done by the caller
            println!("Error parsing line: {:?}", line);
        }
    }
}

async fn handle_error(event_sender: &mpsc::Sender<EngineStateInfoEvent>, e: std::io::Error) {
    let _ = event_sender
        .send(EngineStateInfoEvent::Error(EngineError::from(e)))
        .await;
}

unsafe impl<S, E> Send for OutputHandler<S, E>
where
    S: EngineState + Send,
    E: Send + Sync + Clone + std::fmt::Debug + 'static,
{
}
