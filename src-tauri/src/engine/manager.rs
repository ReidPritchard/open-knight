use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

use crate::parse::uci::OptionDefinition;

use super::events::{EngineStateInfoEvent, EventBus};
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
        app_handle: AppHandle,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut engine = EngineProcess::builder()
            .command(Command::new(path))
            .state(EngineStateInfo::default())
            .build();

        engine
            .spawn(Box::new(UciProtocolParser), Box::new(UciProtocolComposer))
            .await?;

        // Wait for the engine to initialize
        match engine.wait_until_ready(EngineReadyState::Initialized).await {
            Ok(_) => println!("Engine has been initialized"),
            Err(e) => {
                println!("Engine initialization failed: {:?}", e);
                return Err(Box::new(e));
            }
        }

        // Start debounced event emission to frontend
        let event_bus = engine.event_bus();
        if let Ok(event_bus) = event_bus {
            Self::spawn_debounced_event_emitter(name.to_string(), event_bus, app_handle.clone());
        } else {
            println!("Failed to get event bus for engine: {}", name);
        }

        self.engines.insert(name.to_string(), engine);
        self.engine_names.push(name.to_string());

        Ok(())
    }

    fn spawn_debounced_event_emitter(
        engine_name: String,
        event_bus: &EventBus,
        app_handle: AppHandle,
    ) {
        let mut rx = event_bus.subscribe::<EngineStateInfoEvent>();
        tokio::spawn(async move {
            println!("Spawned event emitter for engine: {}", engine_name);
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
}

/// Engine Manager - Public engine management interface
impl EngineManager {
    /// Get a specific engine by name
    pub fn get_engine(&self, name: &str) -> Option<&EngineProcess<EngineStateInfo>> {
        self.engines.get(name)
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
                println!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match set_option_result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Failed to set option: {:?}", e);
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
        println!("Setting position for engines");
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
        println!("Starting analysis for engines");
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
                println!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match set_pos_result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Failed to set position: {:?}", e);
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
                println!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match start_analysis_result {
            Ok(_) => {
                println!("Started analysis for engine: {}", name);
                Ok(())
            }
            Err(e) => {
                println!("Failed to start analysis: {:?}", e);
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
                println!("Failed to get input handler: {:?}", e);
                return Err(Box::new(e));
            }
        };

        match stop_analysis_result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Failed to stop analysis: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}
