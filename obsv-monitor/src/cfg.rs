//! Configuration

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    export::{
        file::FileExporterConfig,
        stdout::{StdoutExporter, StdoutExporterConfig},
    },
    monitor::http::HttpMonitorConfig,
    MonitoringService,
};

/// Monitoring config
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Monitors
    pub monitors: MonitorsConfig,
    /// Exporters
    pub exporters: ExportersConfig,
}

impl MonitoringConfig {
    /// Loads the configuration from a TOML file
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str::<Self>(&content)?)
    }

    /// Returns a service instance
    pub fn service(&self) -> Result<MonitoringService, Error> {
        let mut service = MonitoringService::new();

        // HTTP monitors
        if let Some(cfg_vec) = &self.monitors.http {
            for cfg in cfg_vec {
                service.add_monitor(cfg.to_monitor()?);
            }
        }

        // STDOUT exporter
        if let Some(cfg) = &self.exporters.stdout {
            service.add_exporter(cfg.to_exporter()?);
        }

        // FILE exporter
        if let Some(cfg_vec) = &self.exporters.file {
            for cfg in cfg_vec {
                service.add_exporter(cfg.to_exporter()?);
            }
        }

        let exporter_stdout = StdoutExporter::new("stdout");
        service.add_exporter(exporter_stdout);
        Ok(service)
    }
}

/// Monitors config
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorsConfig {
    /// HTTP monitors
    pub http: Option<Vec<HttpMonitorConfig>>,
}

/// Exporters config
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportersConfig {
    /// Stdout
    pub stdout: Option<StdoutExporterConfig>,
    /// File
    pub file: Option<Vec<FileExporterConfig>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_cfg_to_toml() {
        let cfg = MonitoringConfig {
            monitors: MonitorsConfig {
                http: Some(vec![HttpMonitorConfig {
                    id: "google".to_string(),
                    name: "Google monitor".to_string(),
                    frequency: "5s".to_string(),
                    uri: "http://www.google.com".to_string(),
                    method: Some("GET".to_string()),
                    headers: None,
                }]),
            },
            exporters: ExportersConfig {
                stdout: Some(StdoutExporterConfig {
                    id: "stdout".to_string(),
                }),
                file: Some(vec![FileExporterConfig {
                    id: "file".to_string(),
                    path: "./logs.txt".to_string(),
                }]),
            },
        };
        match toml::to_string(&cfg) {
            Ok(ok) => {
                eprintln!("{ok}")
            }
            Err(err) => {
                panic!("{err:#?}")
            }
        }
    }

    #[test]
    fn test_monitor_cfg_from_toml() {
        let cfg_str = r#"
        [[monitors.http]]
        id = "google"
        name = "Google monitor"
        frequency = "5s"
        uri = "http://www.google.com"
        method = "GET"

        [exporters.stdout]
        id = "stdout"

        [[exporters.file]]
        id = "file"
        path = "./logs.txt"
        "#;
        match toml::from_str::<MonitoringConfig>(cfg_str) {
            Ok(ok) => {
                eprintln!("{ok:#?}")
            }
            Err(err) => {
                panic!("{err:#?}")
            }
        }
    }
}
