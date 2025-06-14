use bevy::prelude::*;
use std::path::PathBuf;
use tracing::{error, instrument, warn};

trait BuoyConfig {
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> where Self: Sized {}

    #[instrument(skip(self))]
    fn validate(&self) -> Result<(), ConfigError> {}    
}

#[derive(Debug)]
pub enum ConfigError {
    #[error("Failed to load config from {path}: {source}")]
    LoadError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Invalid config: {reason}")]
    ValidationError { reason: String },
}