//! Clickhouse exporter

use async_trait::async_trait;
use clickhouse_client::{http::Client, schema::prelude::*};
use serde::Serialize;
use time::OffsetDateTime;

use crate::{error::Error, monitor::MonitorCheck};

use super::Exporter;

/// Clickhouse exporter
pub struct ClickhouseExporter {
    /// ID
    pub id: String,
    /// URL
    pub url: String,
    /// Database
    pub db: String,
    /// DB Client
    pub client: Client,
}

impl ClickhouseExporter {
    /// Creates a new [StdoutExporter]
    pub fn new(id: &str, url: &str, db: &str) -> Self {
        let client = Client::new(url).database(db);

        Self {
            id: id.to_string(),
            url: url.to_string(),
            db: db.to_string(),
            client,
        }
    }

    /// Adds the credentials
    pub fn credentials(mut self, username: &str, password: &str) -> Self {
        self.client = self.client.credentials(username, password);
        self
    }
}

impl From<clickhouse_client::error::Error> for Error {
    fn from(value: clickhouse_client::error::Error) -> Self {
        Error::new(value.to_string().as_str())
    }
}

/// A monitor check in the database
#[derive(Debug, Clone, Serialize, DbRow)]
#[db(table = "monitors")]
struct MonitorCheckDb {
    /// ID
    #[db(primary, type = "UUID")]
    pub id: String,
    /// Monitor ID
    pub monitor: String,
    /// Timestamp
    pub ts: OffsetDateTime,
    /// Status (0: running, 1: OK, -1: error)
    pub status: i8,
    /// Duration (in us)
    pub duration: Option<u128>,
    /// Error
    pub error: Option<String>,
}

impl Default for MonitorCheckDb {
    fn default() -> Self {
        Self {
            id: "0".to_string(),
            monitor: Default::default(),
            ts: OffsetDateTime::UNIX_EPOCH,
            status: 0,
            duration: None,
            error: None,
        }
    }
}

impl From<&MonitorCheck> for MonitorCheckDb {
    fn from(value: &MonitorCheck) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let monitor = value.monitor.clone();
        let ts = value.timestamp;
        let (status, duration, error) = match &value.status {
            crate::monitor::MonitorCheckStatus::Started => (0, None, None),
            crate::monitor::MonitorCheckStatus::Success { duration } => {
                (1, Some(duration.as_micros()), None)
            }
            crate::monitor::MonitorCheckStatus::Failure { duration, message } => {
                (-1, Some(duration.as_micros()), Some(message.clone()))
            }
        };

        Self {
            id,
            monitor,
            ts,
            status,
            duration,
            error,
        }
    }
}

#[async_trait]
impl Exporter for ClickhouseExporter {
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn export(&self, check: &MonitorCheck) -> Result<(), Error> {
        let check: MonitorCheckDb = check.into();
        let _ = self.client.insert(&[check]).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tokio::sync::OnceCell;

    static EXPORTER: OnceCell<ClickhouseExporter> = OnceCell::const_new();

    async fn init() -> &'static ClickhouseExporter {
        crate::tests::init_tracer();

        EXPORTER
            .get_or_init(|| async {
                let exporter =
                    ClickhouseExporter::new("test_monitor", "http://localhost:8123", "test");

                exporter
                    .client
                    .create_table(&MonitorCheckDb::db_schema())
                    .await
                    .unwrap();

                exporter
            })
            .await
    }

    #[tokio::test]
    async fn test_exporter_clickhouse() {
        let exporter = init().await;

        let mut check = MonitorCheck::start("test_monitor");
        check.succeeded();
        exporter.export(&check).await.unwrap();

        let mut check = MonitorCheck::start("test_monitor");
        check.failed("dummy error");
        exporter.export(&check).await.unwrap();
    }
}
