//! Fil exporter

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use time::macros::format_description;

use crate::{error::Error, monitor::MonitorCheck};

use super::Exporter;

/// File exporter
pub struct FileExporter {
    /// ID
    pub id: String,
    /// File path
    pub path: PathBuf,
}

impl FileExporter {
    /// Creates a new [FileExporter]
    pub fn new(id: &str, path: &Path) -> Self {
        Self {
            id: id.to_string(),
            path: path.to_owned(),
        }
    }

    /// Returns the file path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[async_trait]
impl Exporter for FileExporter {
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn export(&self, check: &MonitorCheck) -> Result<(), Error> {
        let ts_fmt = format_description!("[hour]:[minute]:[second]");
        let msg = format!(
            "{} [{}] {}\n",
            check.timestamp.format(ts_fmt).unwrap(),
            check.monitor,
            check.status,
        );

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.path)
            .map_err(|err| Error::new(format!("{err}").as_str()))?;
        let _n = file
            .write(msg.as_bytes())
            .map_err(|err| Error::new(format!("{err}").as_str()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exporter_file() {
        let exp_path = std::env::temp_dir().join("test_exporter_file.txt");
        let exporter = FileExporter::new("test_monitor", &exp_path);
        eprintln!("file exporter: {}", exporter.path().to_string_lossy());

        let mut check = MonitorCheck::start("test_monitor");
        check.succeeded();
        exporter.export(&check).await.unwrap();

        let mut check = MonitorCheck::start("test_monitor");
        check.failed("dummy error");
        exporter.export(&check).await.unwrap();
    }
}
