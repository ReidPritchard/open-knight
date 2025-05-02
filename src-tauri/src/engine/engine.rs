use core::fmt;

use crate::ts_export;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ts_export!(
    pub struct EngineConfig {
        pub name: String,
        pub author: String,
        pub path: String,
        pub options: Vec<EngineOption>,
    }
);

ts_export!(
    pub struct EngineOption {
        pub name: String,
        pub option_type: String,
        pub value: Option<String>,
        pub default: Option<String>,
        pub min: Option<String>,
        pub max: Option<String>,
        pub description: Option<String>,
        pub var: Option<Vec<String>>,
    }
);

ts_export!(
    pub enum EngineError {
        EngineAlreadyLoaded(String),
        EngineNotFound(String),
        EngineNotLoaded(String),
        EngineFailedToLock(String),
        EngineProcessError(String),
    }
);

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::EngineAlreadyLoaded(name) => write!(f, "Engine '{}' already loaded", name),
            EngineError::EngineNotFound(name) => write!(f, "Engine '{}' not found", name),
            EngineError::EngineNotLoaded(name) => write!(f, "Engine '{}' not loaded", name),
            EngineError::EngineFailedToLock(name) => write!(f, "Failed to lock engine '{}'", name),
            EngineError::EngineProcessError(message) => write!(f, "{}", message),
        }
    }
}

impl From<EngineError> for String {
    fn from(error: EngineError) -> Self {
        error.to_string()
    }
}

impl EngineConfig {
    pub fn new(path: String) -> Self {
        Self {
            path,
            name: "".to_string(),
            author: "".to_string(),
            options: vec![],
        }
    }
}
