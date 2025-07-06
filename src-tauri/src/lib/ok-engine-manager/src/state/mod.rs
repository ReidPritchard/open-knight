use super::utils::EngineError;

pub mod engine_state;

/// Common trait for all engine states regardless of protocol
pub trait EngineState: Send + Sync + 'static + Clone {
    type Update: Send + Sync + Clone + std::fmt::Debug + 'static;
    type Event: Send + Sync + Clone + std::fmt::Debug;

    /// Apply an update to the engine state
    fn apply_update(
        &mut self,
        update: Self::Update,
    ) -> Result<Self::Event, EngineError>;
}
