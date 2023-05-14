//! Monitoring service
//!
//! This crate provides a [MonitoringService] service, which checks monitors at regular intervals,
//! and exports the results to a backend (stdout, DB, etc ...).
//!
//! # Features
//!
//! - **rt-tokio**: Tokio-dependent features
//! - **http**: HTTP monitor
//! - **clickhouse**: Clickhouse exporter

use crate::monitor::Monitor;
use export::Exporter;
use monitor::MonitorCheck;

pub mod cfg;
pub mod error;
pub mod export;
pub mod monitor;

use error::Error;

/// Monitoring service
#[derive(Default)]
pub struct MonitoringService {
    /// Monitors
    pub monitors: Vec<Box<dyn Monitor>>,
    /// Exporters
    pub exporters: Vec<Box<dyn Exporter>>,
}

impl MonitoringService {
    /// Instantiates a new [MonitorService]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a monitor
    pub fn monitor(mut self, monitor: impl Monitor + 'static) -> Self {
        self.monitors.push(Box::new(monitor));
        self
    }

    /// Adds a monitor
    pub fn add_monitor(&mut self, monitor: impl Monitor + 'static) {
        self.monitors.push(Box::new(monitor));
    }

    /// Sets an exporter
    pub fn exporter(mut self, exporter: impl Exporter + 'static) -> Self {
        self.exporters.push(Box::new(exporter));
        self
    }

    /// Adds an exporter
    pub fn add_exporter(&mut self, exporter: impl Exporter + 'static) {
        self.exporters.push(Box::new(exporter));
    }

    /// Performs a test for all services
    pub async fn check_all(&self) -> Vec<MonitorCheck> {
        let mut checks = vec![];
        for monitor in self.monitors.iter() {
            let check = monitor.check().await;
            checks.push(check);
        }
        checks
    }

    /// Starts the service
    #[cfg(feature = "rt-tokio")]
    pub async fn start(self) -> Result<(), Error> {
        use duration_string::DurationString;
        use tokio::sync::broadcast;

        // print the service info
        log::info!("starting monitoring service",);
        for monitor in &self.monitors {
            log::info!(
                "monitor: {} (every {})",
                monitor.id(),
                DurationString::from(monitor.frequency()).to_string()
            );
        }
        for exporter in &self.exporters {
            log::info!("exporter: {}", exporter.id());
        }

        // setup a channel to send checks to exporters
        let (tx, mut _rx) = broadcast::channel::<MonitorCheck>(100);

        // setup the monitors tasks
        let mut handles = Vec::new();
        for monitor in self.monitors {
            let tx = tx.clone();

            let handle = tokio::spawn(async move {
                let mut interval = tokio::time::interval(monitor.frequency());

                loop {
                    interval.tick().await;
                    log::trace!("[monitor:{}] trigger", monitor.id());
                    let check = monitor.check().await;
                    log::trace!("[monitor:{}] result: {}", monitor.id(), check.status);
                    tx.send(check).unwrap();
                }
            });
            handles.push(handle);
        }

        // setup the exporters tasks
        for exporter in self.exporters {
            let mut rx = tx.subscribe();

            let handle = tokio::spawn(async move {
                loop {
                    let check = rx.recv().await.unwrap();
                    log::trace!("[exporter:{}] exporting: {}", exporter.id(), check.status);
                    match exporter.export(&check).await {
                        Ok(_) => {
                            log::trace!("[exporter:{}] exported: {}", exporter.id(), check.status);
                        }
                        Err(err) => {
                            log::error!("[exporter:{}] ERROR: {}", exporter.id(), err.to_string());
                        }
                    };
                }
            });
            handles.push(handle);
        }

        // wait for all handles
        for handle in handles {
            match handle.await {
                Ok(_) => {}
                Err(err) => return Err(Error::new(err.to_string().as_str())),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use tracing_ext::sub::PrettyConsoleLayer;
    use tracing_subscriber::{prelude::*, EnvFilter};

    static INIT: Once = Once::new();

    /// Initializes the tracer
    pub(crate) fn init_tracer() {
        INIT.call_once(|| {
            let layer_filter = EnvFilter::from_default_env();
            let layer_console = PrettyConsoleLayer::default()
                .wrapped(true)
                .oneline(false)
                .events_only(false)
                .show_time(false)
                .show_target(true)
                .show_span_info(false)
                .indent(6);
            tracing_subscriber::registry()
                .with(layer_console)
                .with(layer_filter)
                .init();
        });
    }
}
