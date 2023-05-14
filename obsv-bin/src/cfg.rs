//! Configuration

use std::{fs, path::Path};

use anyhow::Result;
use obsv_monitor::cfg::MonitoringConfig;
use serde::{Deserialize, Serialize};

/// Overall configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Monitoring
    pub monitoring: MonitoringConfig,
}

impl Config {
    /// Loads the configuration from a TOML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
