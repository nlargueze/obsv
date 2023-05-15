//! Stdout exporter

use async_trait::async_trait;
use obsv_core::Data;

use super::Exporter;

/// Stdout exporter
#[derive(Debug, Default)]
pub struct StdoutExporter {}

impl StdoutExporter {
    /// Creates a new [StdoutExporter]
    pub fn new() -> Self {
        StdoutExporter::default()
    }
}

#[async_trait]
impl Exporter for StdoutExporter {
    async fn export(&self, data: Vec<Data>) {
        log::trace!("exporting");
        for d in data {
            eprintln!("{d}");
        }
    }
}
