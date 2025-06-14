use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, error, instrument, warn};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load config from {path}: {source}")]
    LoadError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("Invalid config: {reason}")]
    ValidationError { reason: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub physics: PhysicsConfig,
    pub atmosphere: AtmosphereConfig,
    pub materials: Vec<MaterialConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsConfig {
    pub timestep: f32,
    pub gravity: f32,
    pub max_entities: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtmosphereConfig {
    pub sea_level_pressure: f32,
    pub sea_level_temperature: f32,
    pub temperature_lapse_rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialConfig {
    pub name: String,
    pub max_temperature: f32,
    pub density: f32,
    pub emissivity: f32,
    pub absorptivity: f32,
    pub thermal_conductivity: f32,
    pub specific_heat: f32,
    pub poissons_ratio: f32,
    pub elasticity: f32,
    pub max_strain: f32,
    pub max_stress: f32,
}

impl SimulationConfig {
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, ConfigError> {
        debug!("Loading simulation config");
        
        let contents = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            error!(
                error = ?e,
                path = %path.as_ref().display(),
                "Failed to read config file"
            );
            ConfigError::LoadError {
                path: path.as_ref().to_path_buf(),
                source: e,
            }
        })?;

        let config: SimulationConfig = serde_json::from_str(&contents)
            .map_err(|e| {
                error!(
                    error = ?e,
                    path = %path.as_ref().display(),
                    "Failed to parse config file"
                );
                ConfigError::ValidationError {
                    reason: e.to_string(),
                }
            })?;

        debug!("Config loaded successfully, validating...");
        config.validate()?;
        debug!("Config validation successful");
        Ok(config)
    }

    #[instrument(skip(self))]
    fn validate(&self) -> Result<(), ConfigError> {
        if self.physics.timestep <= 0.0 {
            warn!(timestep = self.physics.timestep, "Invalid physics timestep");
            return Err(ConfigError::ValidationError {
                reason: "Physics timestep must be positive".to_string(),
            });
        }

        if self.physics.gravity <= 0.0 {
            warn!(gravity = self.physics.gravity, "Invalid gravity value");
            return Err(ConfigError::ValidationError {
                reason: "Gravity must be positive".to_string(),
            });
        }

        // Validate material properties
        for material in &self.materials {
            if material.max_temperature <= 0.0 {
                warn!(
                    material = %material.name,
                    max_temperature = material.max_temperature,
                    "Invalid material max temperature"
                );
                return Err(ConfigError::ValidationError {
                    reason: format!("Material {}: max_temperature must be positive", material.name),
                });
            }
            if material.density <= 0.0 {
                warn!(
                    material = %material.name,
                    density = material.density,
                    "Invalid material density"
                );
                return Err(ConfigError::ValidationError {
                    reason: format!("Material {}: density must be positive", material.name),
                });
            }
            if !(0.0..=1.0).contains(&material.emissivity) {
                warn!(
                    material = %material.name,
                    emissivity = material.emissivity,
                    "Invalid material emissivity"
                );
                return Err(ConfigError::ValidationError {
                    reason: format!("Material {}: emissivity must be between 0 and 1", material.name),
                });
            }
        }

        debug!("All material properties validated successfully");
        Ok(())
    }
} 