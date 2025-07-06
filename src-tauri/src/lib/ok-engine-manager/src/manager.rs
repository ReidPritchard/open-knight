use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command;

use ok_parse::uci::OptionDefinition;

use crate::events::EventEmitter;

use super::events::{EngineStateInfoEvent, EventBus};
use super::utils::EngineError;
use super::{
    process::EngineProcess,
    protocol::{
        uci_protocol::{UciProtocolComposer, UciProtocolParser},
        OptionValue,
    },
    state::engine_state::{EngineReadyState, EngineStateInfo},
};

/// Time management strategies for analysis
///
/// For a single position analysis, total and fixed result in the same behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeStrategy {
    /// Total time budget distributed across all moves
    TotalBudget {
        total_seconds: u64,
    },
    // Fixed time per move
    FixedPerMove {
        seconds_per_move: u64,
    },
    /// Fixed depth per move
    FixedDepth {
        depth: u32,
    },
}

impl Default for TimeStrategy {
    fn default() -> Self {
        TimeStrategy::TotalBudget { total_seconds: 60 }
    }
}

/// Engine analysis configuration for a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineAnalysisConfig {
    /// Time management strategy
    pub time_strategy: TimeStrategy,
    /// Include variations in the analysis
    pub include_variations: bool,
    /// The name of the engine to use
    pub engine_name: String,
}

impl Default for EngineAnalysisConfig {
    fn default() -> Self {
        Self {
            time_strategy: TimeStrategy::default(),
            include_variations: false,
            engine_name: "stockfish".to_string(),
        }
    }
}

/// A struct for managing multiple engine processes
///
/// An interface for easily managing multiple engine processes
/// Currently the manager only uses EngineStateInfo as the engine state
/// It's possible this will change in the future to support more complex
/// or unique engine states
pub struct EngineManager<Emitter>
where
    Emitter: EventEmitter + Send + Sync + 'static,
{
    engines: HashMap<String, EngineProcess<EngineStateInfo>>,
    engine_names: Vec<String>,
    event_emitter: Option<Arc<Emitter>>,
}

impl<Emitter> Default for EngineManager<Emitter>
where
    Emitter: EventEmitter + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Emitter> EngineManager<Emitter>
where
    Emitter: EventEmitter + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
            engine_names: Vec::new(),
            event_emitter: None,
        }
    }

    /// Create a new EngineManager with an event emitter
    pub fn with_emitter(event_emitter: Arc<Emitter>) -> Self {
        Self {
            engines: HashMap::new(),
            engine_names: Vec::new(),
            event_emitter: Some(event_emitter),
        }
    }

    /// Add a new UCI engine to the manager
    ///
    /// Path is the path to the engine executable
    pub async fn add_uci_engine(
        &mut self,
        name: &str,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Improve check if the exact engine is already loaded
        // If it is, return an error
        if self.engines.contains_key(name) {
            // return Err(Box::new(EngineError::EngineAlreadyRunning(
            //     name.to_string(),
            // )));
            // Don't return an error, just don't try to load it
            return Ok(());
        }

        let mut command = Command::new(path);
        command.kill_on_drop(true);
        let cmd = command;

        let mut engine = EngineProcess::builder()
            .command(cmd)
            .state(EngineStateInfo::default())
            .build();

        engine
            .spawn(Box::new(UciProtocolParser), Box::new(UciProtocolComposer))
            .await?;

        // Wait for the engine to initialize
        match engine.wait_until_ready(EngineReadyState::Initialized).await {
            Ok(_) => {
                info!("Engine has been initialized");

                // Emit the engine options
                let options = engine
                    .query_state(|state| state.capabilities.clone())
                    .await;
                if let Ok(options_payload) =
                    serde_json::to_string(&(name.to_string(), options))
                {
                    if let Some(ref emitter) = self.event_emitter {
                        emitter.emit_event("engine-options", options_payload);
                    }
                }
            }
            Err(e) => {
                error!("Engine initialization failed: {:?}", e);
                return Err(Box::new(e));
            }
        }

        // Start debounced event emission
        let event_bus = engine.event_bus();
        if let Ok(event_bus) = event_bus {
            if let Some(ref emitter) = self.event_emitter {
                Self::spawn_debounced_event_emitter(
                    name.to_string(),
                    event_bus,
                    emitter.clone(),
                );
            }
        } else {
            error!("Failed to get event bus for engine: {}", name);
        }

        self.engines.insert(name.to_string(), engine);
        self.engine_names.push(name.to_string());

        Ok(())
    }

    fn spawn_debounced_event_emitter(
        engine_name: String,
        event_bus: &EventBus,
        event_emitter: Arc<Emitter>,
    ) {
        let mut rx = event_bus.subscribe::<EngineStateInfoEvent>();
        tokio::spawn(async move {
            info!("Spawned event emitter for engine: {}", engine_name);
            // let mut last_event: Option<EngineStateInfoEvent> = None;
            // let mut ticker = interval(Duration::from_millis(100));
            loop {
                // TODO: Implement debouncing
                // tokio::select! {
                //     Some(event) = rx.recv() => {
                //         println!("Engine event: {:?}", event);
                //         last_event = Some(event);
                //     }
                //     _ = ticker.tick() => {
                //         if let Some(event) = last_event.take() {
                //             if let Ok(payload) = serde_json::to_string(&(engine_name.clone(), event)) {
                //                 event_emitter.emit("engine-output", payload);
                //             }
                //         }
                //     }
                // }

                // For now just emit every event
                let event = rx.recv().await;
                if let Some(event) = event {
                    if let Ok(payload) =
                        serde_json::to_string(&(engine_name.clone(), event))
                    {
                        event_emitter.emit_event("engine-output", payload);
                    }
                }
            }
        });
    }

    pub async fn remove_engine(
        &mut self,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.engines.remove(name);
        if let Some(mut engine) = engine {
            let kill_result = engine.kill(None).await;
            match kill_result {
                Ok(_) => {
                    info!("Engine killed: {}", name);
                    // Clean up the engine state
                    let _ = self.engines.remove(name);
                    let _ = self.engine_names.remove(
                        self.engine_names
                            .iter()
                            .position(|x| x == name)
                            .unwrap(),
                    );
                }
                Err(e) => {
                    error!("Failed to kill engine: {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }
        Ok(())
    }
}

/// Engine Manager - Public engine management interface
impl<Emitter> EngineManager<Emitter>
where
    Emitter: EventEmitter + Send + Sync + 'static,
{
    /// Get a specific engine by name
    pub fn get_engine(
        &self,
        name: &str,
    ) -> Option<&EngineProcess<EngineStateInfo>> {
        self.engines.get(name)
    }

    /// Get all engines and their state
    ///
    /// Used to initialize the UI, might be removed in the future
    /// as it's mostly needed when HMR is used
    pub async fn get_all_engine_state(&self) -> Vec<(String, EngineStateInfo)> {
        let mut states = Vec::new();
        for (name, engine) in self.engines.iter() {
            let state = engine.query_state(|state| state.clone()).await;
            states.push((name.clone(), state));
        }
        states
    }

    /// Get the options/capabilities for a specific engine
    pub async fn get_engine_capabilities(
        &self,
        name: &str,
    ) -> Option<HashMap<String, OptionDefinition>> {
        if let Some(engine) = self.engines.get(name) {
            let capabilities =
                engine.query_state(|state| state.capabilities.clone()).await;
            Some(capabilities)
        } else {
            None
        }
    }

    /// Set an option or capability for a specific engine
    pub async fn set_engine_option(
        &mut self,
        name: &str,
        option: &str,
        value: OptionValue,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.engines.get_mut(name).unwrap();
        let set_option_result = match engine.input_handler() {
            Ok(handler) => handler.set_option(option, value).await,
            Err(e) => {
                error!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match set_option_result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to set option: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Set the position for all engines
    pub async fn set_position(
        &mut self,
        fen: Option<&str>,
        moves: Option<&[&str]>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Setting position for engines");
        let engine_names: Vec<_> = self.engine_names.clone();
        for engine_name in engine_names.iter() {
            let result =
                self.set_engine_position(engine_name, fen, moves).await;
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    /// Start analysis for all engines
    pub async fn start_position_analysis(
        &mut self,
        depth: Option<u32>,
        time_ms: Option<u32>,
        multipv: Option<u32>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting analysis for engines");
        let engine_names: Vec<_> = self.engine_names.clone();
        for engine_name in engine_names.iter() {
            let result = self
                .start_engine_analysis(engine_name, depth, time_ms, multipv)
                .await;
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    /// Set and start position analysis for a specific engine
    pub async fn quick_start_position_analysis_for(
        &mut self,
        engine_name: &str,
        position_fen: &str,
        depth: Option<u32>,
        time_ms: Option<u32>,
        multipv: Option<u32>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Ensure the engine exists
        let engine = self.engines.get_mut(engine_name);
        if engine.is_none() {
            return Err("Engine not found".into());
        }

        // Set the position
        self.set_engine_position(engine_name, Some(position_fen), None)
            .await?;

        // Start analysis
        self.start_engine_analysis(engine_name, depth, time_ms, multipv)
            .await
    }

    /// Stop analysis for all engines
    pub async fn stop_analysis(
        &mut self
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine_names: Vec<_> = self.engine_names.clone();
        for engine_name in engine_names.iter() {
            let result = self.stop_engine_analysis(engine_name).await;
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }
}

/// Engine Manager - Internal engine management interface
impl<Emitter> EngineManager<Emitter>
where
    Emitter: EventEmitter + Send + Sync,
{
    /// Set the position for a specific engine
    async fn set_engine_position(
        &mut self,
        name: &str,
        fen: Option<&str>,
        moves: Option<&[&str]>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.engines.get_mut(name).unwrap();
        // Set the position to a FEN string
        let set_pos_result = match engine.input_handler() {
            Ok(handler) => handler.set_position(fen, moves).await,
            Err(e) => {
                error!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match set_pos_result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to set position: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Start analysis for a specific engine
    async fn start_engine_analysis(
        &mut self,
        name: &str,
        depth: Option<u32>,
        time_ms: Option<u32>,
        multipv: Option<u32>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.engines.get_mut(name).unwrap();
        let start_analysis_result = match engine.input_handler() {
            Ok(handler) => {
                handler.start_analysis(depth, time_ms, multipv).await
            }
            Err(e) => {
                error!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match start_analysis_result {
            Ok(_) => {
                info!("Started analysis for engine: {}", name);
                Ok(())
            }
            Err(e) => {
                error!("Failed to start analysis: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Stop analysis for a specific engine
    async fn stop_engine_analysis(
        &mut self,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.engines.get_mut(name).unwrap();
        let stop_analysis_result = match engine.input_handler() {
            Ok(handler) => handler.stop_analysis().await,
            Err(e) => {
                error!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match stop_analysis_result {
            Ok(_) => {
                info!("Stopped analysis for engine: {}", name);
                Ok(())
            }
            Err(e) => {
                error!("Failed to stop analysis: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}
