use crate::ts_export;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ts_export!(
    pub struct Engine {
        pub name: String,
        pub version: String,
        pub path: String,
        pub options: Vec<EngineOption>,
    }
);

ts_export!(
    pub struct EngineOption {
        pub name: String,
        pub value: String,
        pub default: String,
        pub min: String,
        pub max: String,
        pub description: String,
    }
);

impl Engine {
    pub fn new(name: String, path: String) -> Self {
        Self {
            path,
            name,
            version: "1.0.0".to_string(),
            options: vec![],
        }
    }
}
