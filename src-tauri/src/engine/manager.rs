use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::process::Command;

use ok_parse::uci::OptionDefinition;

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

/// A struct for managing multiple engine processes
///
/// An interface for easily managing multiple engine processes
/// Currently the manager only uses EngineStateInfo as the engine state
/// It's possible this will change in the future to support more complex
/// or unique engine states
pub struct EngineManager {
    engines: HashMap<String, EngineProcess<EngineStateInfo>>,
    engine_names: Vec<String>,
}

impl Default for EngineManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineManager {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
            engine_names: Vec::new(),
        }
    }

    /// Add a new UCI engine to the manager
    ///
    /// Path is the path to the engine executable
    pub async fn add_uci_engine(
        &mut self,
        name: &str,
        path: &str,
        app_handle: Arc<AppHandle>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Improve check if the exact engine is already loaded
        // If it is, return an error
        if self.engines.contains_key(name) {
            return Err(Box::new(EngineError::EngineAlreadyRunning(
                name.to_string(),
            )));
        }

        let mut engine = EngineProcess::builder()
            .command(Command::new(path))
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
                let options = engine.query_state(|state| state.capabilities.clone()).await;
                if let Ok(options_payload) = serde_json::to_string(&(name.to_string(), options)) {
                    let _ = app_handle.emit("engine-options", options_payload);
                }
            }
            Err(e) => {
                error!("Engine initialization failed: {:?}", e);
                return Err(Box::new(e));
            }
        }

        // Start debounced event emission to frontend
        let event_bus = engine.event_bus();
        if let Ok(event_bus) = event_bus {
            Self::spawn_debounced_event_emitter(name.to_string(), event_bus, app_handle.clone());
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
        app_handle: Arc<AppHandle>,
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
                //                 let _ = app_handle.emit("engine-output", payload);
                //             }
                //         }
                //     }
                // }

                // For now just emit every event
                let event = rx.recv().await;
                if let Some(event) = event {
                    if let Ok(payload) = serde_json::to_string(&(engine_name.clone(), event)) {
                        let _ = app_handle.emit("engine-output", payload);
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
                    let _ = self
                        .engine_names
                        .remove(self.engine_names.iter().position(|x| x == name).unwrap());
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
impl EngineManager {
    /// Get a specific engine by name
    pub fn get_engine(&self, name: &str) -> Option<&EngineProcess<EngineStateInfo>> {
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
            let capabilities = engine.query_state(|state| state.capabilities.clone()).await;
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
            let result = self.set_engine_position(engine_name, fen, moves).await;
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    /// Start analysis for all engines
    pub async fn start_analysis(
        &mut self,
        depth: Option<u32>,
        time_ms: Option<u32>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting analysis for engines");
        let engine_names: Vec<_> = self.engine_names.clone();
        for engine_name in engine_names.iter() {
            let result = self
                .start_engine_analysis(engine_name, depth, time_ms)
                .await;
            if result.is_err() {
                return Err(result.unwrap_err());
            }
        }

        Ok(())
    }

    /// Stop analysis for all engines
    pub async fn stop_analysis(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
impl EngineManager {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.engines.get_mut(name).unwrap();
        let start_analysis_result = match engine.input_handler() {
            Ok(handler) => handler.start_analysis(depth, time_ms).await,
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
