use bevy::prelude::*;
use std::path::PathBuf;
use tracing::{error, instrument, warn};

trait BuoyConfig {
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> where Self: Sized {
        todo!()
    }

    #[instrument(skip(self))]
    fn validate(&self) -> Result<(), ConfigError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    LoadError {
        path: PathBuf,
        source: std::io::Error,
    },
    ValidationError { reason: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::LoadError { path, source } => {
                error!(
                    "Failed to load config from {}: {}",
                    path.display(),
                    source
                );
                write!(f, "Failed to load config from {}: {}", path.display(), source)
            }
            ConfigError::ValidationError { reason } => {
                error!("Invalid config: {}", reason);
                write!(f, "Invalid config: {}", reason)
            }
        }
    }
}
