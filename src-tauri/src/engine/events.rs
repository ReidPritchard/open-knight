use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;
use tokio::sync::mpsc;
use tokio::time::Instant;

use open_knight_parse::uci::IdInfo;

use super::state::EngineState;
use super::utils::EngineError;

/// Common interface for all engine lifecycle events
#[derive(Debug, Clone, Serialize)]
pub enum LifecycleEvent {
    EngineStarted,
    EngineStopped,
    EnginePaused,
    EngineResumed,
}

/// A generic state change event
#[derive(Debug, Clone)]
pub struct StateChange<S: EngineState> {
    pub field: String,
    pub update: S::Update,
    pub timestamp: Instant,
}

/// Engine state change events specific to EngineStateInfo
#[derive(Debug, Clone, Serialize)]
pub enum EngineStateInfoEvent {
    /// Engine Info Update
    InfoUpdate(IdInfo),
    /// Adding an engine capability/config option
    CapabilityAdded(String, open_knight_parse::uci::OptionDefinition),
    /// Update of the engine's analysis
    AnalysisUpdate(open_knight_parse::uci::InfoParams),
    /// Update of the engine's best move (Best Move, Ponder Move)
    BestMove(String, Option<String>),
    /// Update of the engine's ready state
    ReadyStateChanged(super::state::engine_state::EngineReadyState),
    /// Update of the engine's current position
    CurrentPositionChanged(String),
    /// Lifecycle event (start, stop, pause, resume)
    LifecycleEvent(LifecycleEvent),
    /// Error event
    Error(EngineError),
}

/// Trait to handle the sending of typed events
trait TypedSender: Send + Sync {
    /// Check if the sender's channel is closed
    fn is_closed(&self) -> bool;

    /// Send a boxed event to the channel
    fn send_boxed(&self, event: Box<dyn Any + Send>) -> bool;
}

/// A typed sender for a specific event type
struct EventSender<T: Clone + Send + 'static> {
    sender: mpsc::Sender<T>,
}

impl<T: Clone + Send + 'static> TypedSender for EventSender<T> {
    fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }

    fn send_boxed(&self, event: Box<dyn Any + Send>) -> bool {
        // Try to downcast the boxed event to the expected type
        if let Ok(typed_event) = event.downcast::<T>() {
            // Use try_send to avoid blocking
            let _ = self.sender.try_send(*typed_event);
            true
        } else {
            false
        }
    }
}

/// Event bus for publishing and subscribing to typed events
pub struct EventBus {
    subscribers: Mutex<HashMap<TypeId, Vec<Box<dyn TypedSender + Send + Sync>>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(HashMap::new()),
        }
    }

    /// Broadcast an event to all subscribers of that event type
    pub fn publish<T: Clone + Send + 'static>(&self, event: T) {
        let type_id = TypeId::of::<T>();
        let mut subscribers = self.subscribers.lock().unwrap();

        if let Some(senders) = subscribers.get_mut(&type_id) {
            // Clean up closed senders
            senders.retain(|sender| !sender.is_closed());

            // Clone and send the event to all remaining senders
            for sender in senders.iter() {
                let event_clone = event.clone();
                let boxed_event: Box<dyn Any + Send> = Box::new(event_clone);
                let _ = sender.send_boxed(boxed_event);
            }
        }
    }

    /// Subscribe to events of a specific type
    pub fn subscribe<T: Clone + Send + 'static>(&self) -> mpsc::Receiver<T> {
        // Create a channel with a buffer size of 100
        let (sender, receiver) = mpsc::channel::<T>(100);

        // Create a typed sender
        let event_sender = EventSender { sender };
        let boxed_sender: Box<dyn TypedSender + Send + Sync> = Box::new(event_sender);

        // Add the sender to the subscribers map
        let type_id = TypeId::of::<T>();
        let mut subscribers = self.subscribers.lock().unwrap();

        subscribers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(boxed_sender);

        receiver
    }

    /// Subscribe with a custom buffer size
    pub fn subscribe_with_buffer<T: Clone + Send + 'static>(
        &self,
        buffer_size: usize,
    ) -> mpsc::Receiver<T> {
        // Create a channel with the specified buffer size
        let (sender, receiver) = mpsc::channel::<T>(buffer_size);

        // Create a typed sender
        let event_sender = EventSender { sender };
        let boxed_sender: Box<dyn TypedSender + Send + Sync> = Box::new(event_sender);

        // Add the sender to the subscribers map
        let type_id = TypeId::of::<T>();
        let mut subscribers = self.subscribers.lock().unwrap();

        subscribers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(boxed_sender);

        receiver
    }

    /// Check if there are any subscribers for a specific event type
    pub fn has_subscribers<T: Send + 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        let subscribers = self.subscribers.lock().unwrap();

        if let Some(senders) = subscribers.get(&type_id) {
            // Check if there are any non-closed senders
            senders.iter().any(|sender| !sender.is_closed())
        } else {
            false
        }
    }

    /// Get the number of subscribers for a specific event type
    pub fn subscriber_count<T: Send + 'static>(&self) -> usize {
        let type_id = TypeId::of::<T>();
        let subscribers = self.subscribers.lock().unwrap();

        if let Some(senders) = subscribers.get(&type_id) {
            // Count the number of non-closed senders
            senders.iter().filter(|sender| !sender.is_closed()).count()
        } else {
            0
        }
    }
}
