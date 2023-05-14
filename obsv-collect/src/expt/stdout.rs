//! Stdout exporter

use async_trait::async_trait;
use obsv_core::Data;

use super::Exporter;

/// Stdout exporter
#[derive(Debug, Default)]
pub struct StdoutExporter {}

impl StdoutExporter {
    pub fn new() -> Self {
        StdoutExporter::default()
    }
}

#[async_trait]
impl Exporter for StdoutExporter {
    fn id(&self) -> String {
        "exporter_stdout".to_string()
    }

    async fn export(&self, data: Data) {
        log::trace!("[{}] Exporting data: {data:?}", self.id());
        eprintln!("{:#?}", data);
    }
}
