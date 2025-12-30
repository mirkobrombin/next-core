use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleType {
    Gaming,
    Software,
    Custom,
}

impl Default for BottleType {
    fn default() -> Self {
        Self::Custom
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BottleConfig {
    pub runner: Option<String>,
    pub dxvk_version: Option<String>,
    pub vkd3d_version: Option<String>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottle {
    pub name: String,
    pub path: PathBuf,
    pub kind: BottleType,
    pub config: BottleConfig,
    #[serde(skip)]
    pub active: bool, // Runtime state, not persisted
}

impl Bottle {
    pub fn new(name: String, path: impl Into<PathBuf>, kind: BottleType) -> Self {
        Self {
            name,
            path: path.into(),
            kind,
            config: BottleConfig::default(),
            active: false,
        }
    }
}
