//! Stdout exporter

use async_trait::async_trait;

use crate::Data;

use super::Exporter;

/// Stdout exporter
#[derive(Debug, Default, Clone)]
pub struct StdoutExporter {}

impl StdoutExporter {
    /// Creates a new [StdoutExporter]
    pub fn new() -> Self {
        StdoutExporter::default()
    }
}

#[async_trait]
impl Exporter for StdoutExporter {
    async fn export(&self, data: &Vec<Data>) {
        log::trace!("exporting");
        for d in data {
            eprintln!("{}", format_data(d));
        }
    }
}

/// Formats Data into a string
fn format_data(data: &Data) -> String {
    todo!("implement data formatting for stdout");
}
