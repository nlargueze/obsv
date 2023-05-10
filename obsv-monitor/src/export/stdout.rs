//! Stdout exporter

use async_trait::async_trait;
use time::macros::format_description;

use crate::{error::Error, monitor::MonitorCheck};

use super::Exporter;

/// Stdout exporter
///
/// A monitor trace is exported as: `[MONITOR] [TS] OK`
pub struct StdoutExporter<T>
where
    T: StdoutFormatter,
{
    /// ID
    pub id: String,
    /// Formatter
    pub formatter: T,
}

/// A trait to implement multplie formatters for [StdoutExporter]
pub trait StdoutFormatter {
    /// Formats a check as a [String]
    fn format(&self, check: &MonitorCheck) -> String;
}

/// Default stdout formatter
pub struct StdoutDefaultFormatter;

impl StdoutFormatter for StdoutDefaultFormatter {
    fn format(&self, check: &MonitorCheck) -> String {
        let ts_fmt = format_description!("[hour]:[minute]:[second]");
        format!(
            "[{}] [{}] {}",
            check.monitor,
            check.timestamp.format(ts_fmt).unwrap(),
            check.status
        )
    }
}

impl StdoutExporter<StdoutDefaultFormatter> {
    /// Creates a new [StdoutExporter]
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            formatter: StdoutDefaultFormatter,
        }
    }
}

impl<T> StdoutExporter<T>
where
    T: StdoutFormatter,
{
    /// Creates a new [StdoutExporter]
    pub fn with_formatter(id: &str, formatter: T) -> Self {
        Self {
            id: id.to_string(),
            formatter,
        }
    }
}

#[async_trait]
impl<T> Exporter for StdoutExporter<T>
where
    T: StdoutFormatter + Send + Sync,
{
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn export(&self, check: &MonitorCheck) -> Result<(), Error> {
        let msg = self.formatter.format(check);
        eprintln!("{}", msg);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exporter_stdout() {
        let exporter = StdoutExporter::new("test_monitor");

        let mut check = MonitorCheck::start("test_monitor");
        check.succeeded();
        exporter.export(&check).await.unwrap();

        let mut check = MonitorCheck::start("test_monitor");
        check.failed("dummy error");
        exporter.export(&check).await.unwrap();
    }
}
