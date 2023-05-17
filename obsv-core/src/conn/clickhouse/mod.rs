//! Clickhouse connector

use clickhouse_client::{orm::prelude::*, schema::DbSchema, Client};

use self::{
    log::DbLog,
    monitor::{DbMonitor, DbMonitorCheck},
    trace::DbSpan,
};

mod attr;
mod log;
mod monitor;
mod trace;

pub use clickhouse_client::query::Where;

#[derive(Debug, thiserror::Error)]
#[error("Clickhouse error: {0}")]
pub struct Error(String);

impl From<clickhouse_client::error::Error> for Error {
    fn from(value: clickhouse_client::error::Error) -> Self {
        Self(value.to_string())
    }
}

/// Clickhouse connector
pub struct ClickhouseConnector {
    /// DB Client
    client: Client,
    /// DB
    db: String,
}

impl ClickhouseConnector {
    /// Creates a new Clickhouse connector
    pub fn new(client: Client) -> Result<Self, Error> {
        let db = if let Some(db) = &client.db {
            db.clone()
        } else {
            return Err(Error("Clickhouse client does not have a DB".to_string()));
        };
        Ok(Self { client, db })
    }

    /// Returns the DB schema for obversability
    pub fn db_schema(&self) -> DbSchema {
        let tbl_traces = DbSpan::db_schema();
        let tbl_logs = DbLog::db_schema();
        let tbl_monitors = DbMonitor::db_schema();
        let tbl_monitors_checks = DbMonitorCheck::db_schema();
        DbSchema::new()
            .table(tbl_traces)
            .table(tbl_logs)
            .table(tbl_monitors)
            .table(tbl_monitors_checks)
    }

    /// Initializes the obversability DB
    pub async fn init_db(&self) -> Result<(), Error> {
        let schema = self.db_schema();
        self.client
            .create_db(&self.db, &schema)
            .await
            .map_err(|err| Error(err.to_string()))?;
        for table in &schema.tables {
            self.client
                .create_table(table, "ReplacingMergeTree")
                .await
                .map_err(|err| Error(err.to_string()))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::OnceCell;

    static ONCE: OnceCell<ClickhouseConnector> = OnceCell::const_new();

    /// Initializes the Clickhouse connector (and tracing)
    pub(crate) async fn init_clickhouse() -> &'static ClickhouseConnector {
        crate::tests::init_tracer();
        ONCE.get_or_init(|| async {
            let client = Client::new("http://localhost:8123").database("obsv");
            let conn = ClickhouseConnector::new(client).unwrap();
            conn.init_db().await.unwrap();
            conn
        })
        .await
    }
}
