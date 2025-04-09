// src-tauri/src/engine/manager.rs
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::engine::uci::UciEngine;

pub struct EngineManager {
    engines: HashMap<String, Arc<Mutex<UciEngine>>>,
    app_handle: AppHandle,
}

impl EngineManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            engines: HashMap::new(),
            app_handle,
        }
    }

    pub fn load_engine(&mut self, name: String, path: PathBuf) -> Result<(), String> {
        if self.engines.contains_key(&name) {
            return Err(format!("Engine '{}' already loaded", name));
        }

        match UciEngine::new(path, self.app_handle.clone()) {
            Ok(engine) => {
                self.engines.insert(name, Arc::new(Mutex::new(engine)));
                Ok(())
            }
            Err(e) => Err(format!("Failed to load engine: {}", e)),
        }
    }

    pub fn get_engine(&self, name: &str) -> Option<Arc<Mutex<UciEngine>>> {
        self.engines.get(name).cloned()
    }

    pub fn unload_engine(&mut self, name: &str) -> Result<(), String> {
        if let Some(engine) = self.engines.remove(name) {
            // Make sure to stop the engine before dropping it
            if let Ok(mut engine) = engine.lock() {
                engine.quit()?;
            }
            Ok(())
        } else {
            Err(format!("Engine '{}' not found", name))
        }
    }

    pub fn list_engines(&self) -> Vec<String> {
        self.engines.keys().cloned().collect()
    }
}
