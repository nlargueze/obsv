//! File exporter

use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use obsv_core::Data;

use super::Exporter;

/// Stdout exporter
#[derive(Debug)]
pub struct FileExporter {
    /// File path
    path: PathBuf,
}

impl FileExporter {
    /// Creates a file exporter
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}

#[async_trait]
impl Exporter for FileExporter {
    async fn export(&self, data: Vec<Data>) {
        log::trace!("exporting");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();
        let content = data
            .into_iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        file.write_all(content.as_bytes()).unwrap();
    }
}
