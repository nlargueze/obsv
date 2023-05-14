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
#[derive(Debug, Default)]
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
    async fn export(&self, data: Data) {
        log::trace!("exporting");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();
        let content = format!("{data:?}");
        file.write_all("\n".as_bytes()).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
