//! Clickhouse

use async_trait::async_trait;
use clickhouse_client::http::Client;

use super::Exporter;

/// Clickhouse exporter
#[derive(Debug, Clone)]
pub struct ClickhouseExporter {
    ///  Clickhouse client
    client: Client,
}

impl ClickhouseExporter {
    /// Creates a new [StdoutExporter]
    pub fn new(addr: &str, db: &str) -> Self {
        let client = Client::new(addr).database(db);
        Self { client }
    }
}

#[async_trait]
impl Exporter for ClickhouseExporter {
    async fn export(&self, data: &Vec<Data>) {
        todo!("implement the clickhouse exporter");
        // log::trace!("exporting to clickhouse");
        // if let Err(err) = self.client.insert(records).await {
        //     log::error!("Failed to export to clickhouse")
        // }
    }
}
