// Configuration module for RACO Core
//
// This module provides functionality for loading, parsing, and accessing configuration.

use anyhow::{Context, Result};
use config::{Config, Environment, File};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

/// Default configuration values
pub const DEFAULT_CONFIG_FILE: &str = "raco.toml";

/// Core configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
    /// Path to data directory
    pub data_dir: PathBuf,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            data_dir: default_data_dir(),
        }
    }
}

/// Returns the default data directory.
/// On Unix, this is typically ~/.local/share/raco
/// On macOS, this is typically ~/Library/Application Support/raco
/// On Windows, this is typically C:\Users\{username}\AppData\Roaming\raco
pub fn default_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| {
            home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".raco")
        })
        .join("raco")
}

/// Loads configuration from default locations
pub fn load_config() -> Result<CoreConfig> {
    let config_path = std::env::var("RACO_CONFIG")
        .ok()
        .map(PathBuf::from)
        .or_else(|| home_dir().map(|h| h.join(".config").join(DEFAULT_CONFIG_FILE)));

    let mut config_builder = Config::builder();

    // Start with defaults
    config_builder =
        config_builder.set_default("data_dir", default_data_dir().to_string_lossy().to_string())?;

    // Load from file if it exists
    if let Some(config_path) = config_path.as_ref() {
        if config_path.exists() {
            info!("Loading config from {}", config_path.display());
            config_builder = config_builder.add_source(File::from(config_path.as_path()));
        }
    }

    // Override with environment variables (e.g., RACO_DATA_DIR)
    config_builder = config_builder.add_source(Environment::with_prefix("RACO").separator("_"));

    // Build the config
    let config = config_builder
        .build()
        .context("Failed to build configuration")?;

    // Convert to our config structure
    let core_config: CoreConfig = config
        .try_deserialize()
        .context("Failed to deserialize configuration")?;

    Ok(core_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_default_data_dir() {
        let data_dir = default_data_dir();
        assert!(data_dir.ends_with("raco"));
    }

    #[rstest]
    fn test_load_default_config() {
        let config = load_config();
        assert!(config.is_ok());
    }
}
