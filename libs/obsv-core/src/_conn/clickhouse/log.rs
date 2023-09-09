//! DB logs

use clickhouse_client::{orm::prelude::*, query::Where};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::Log;

use super::{attr::DbAttrs, Error};

/// A DB log
#[derive(Debug, Clone, DbRecord)]
#[db(table = "logs")]
pub struct DbLog {
    /// ID
    #[db(primary)]
    pub id: Uuid,
    /// Resource name
    pub resource: String,
    /// Resource attributes
    pub resource_attrs: DbAttrs,
    /// Scope name
    pub scope: String,
    /// Resource attributes
    pub scope_attrs: DbAttrs,
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Level (severity)
    pub level: i32,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: DbAttrs,
    /// Trace id
    pub trace_id: u128,
    /// Span id
    pub span_id: u64,
}

impl Default for DbLog {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            resource: String::default(),
            resource_attrs: DbAttrs::default(),
            scope: String::default(),
            scope_attrs: DbAttrs::default(),
            timestamp: OffsetDateTime::UNIX_EPOCH,
            level: 0,
            message: String::default(),
            attrs: DbAttrs::default(),
            trace_id: 0,
            span_id: 0,
        }
    }
}

impl From<crate::Log> for DbLog {
    fn from(value: crate::Log) -> Self {
        Self {
            id: Uuid::new_v4(),
            resource: value.resource,
            resource_attrs: value.resource_attrs.into(),
            scope: value.scope,
            scope_attrs: value.scope_attrs.into(),
            timestamp: value.timestamp,
            level: value.level,
            message: value.message,
            attrs: value.attrs.into(),
            trace_id: value.trace_id,
            span_id: value.span_id,
        }
    }
}

impl From<DbLog> for crate::Log {
    fn from(value: DbLog) -> Self {
        Self {
            resource: value.resource,
            resource_attrs: value.resource_attrs.into(),
            scope: value.scope,
            scope_attrs: value.scope_attrs.into(),
            timestamp: value.timestamp,
            level: value.level,
            message: value.message,
            attrs: value.attrs.into(),
            trace_id: value.trace_id,
            span_id: value.span_id,
        }
    }
}

impl super::ClickhouseConnector {
    /// Inserts logs
    pub async fn insert_logs(&self, logs: &[Log]) -> Result<(), Error> {
        let db_logs = logs
            .iter()
            .map(|m| DbLog::from(m.clone()))
            .collect::<Vec<_>>();
        Ok(self.client.insert(&db_logs).await?)
    }

    /// Returns the logs
    pub async fn get_logs(&self, where_cond: Where) -> Result<Vec<Log>, Error> {
        let db_logs = self.client.select::<DbLog>(&[], where_cond).await?;
        Ok(db_logs.into_iter().map(Log::from).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use time::OffsetDateTime;

    use super::*;
    use crate::Log;

    #[tokio::test]
    async fn test_clickhouse_logs() {
        let conn = crate::conn::clickhouse::tests::init_clickhouse().await;
        let log = Log {
            resource: "test_service".into(),
            resource_attrs: HashMap::new(),
            scope: "test_scope".into(),
            scope_attrs: HashMap::new(),
            timestamp: OffsetDateTime::now_utc(),
            level: 1,
            message: "My message".into(),
            attrs: HashMap::new(),
            trace_id: 1,
            span_id: 1,
        };

        if let Err(err) = conn.insert_logs(&[log]).await {
            tracing::error!(
                test = "test_clickhouse_logs",
                error = err.to_string(),
                "failed to insert"
            );
            panic!("{}", err);
        }
        match conn.get_monitors(Where::empty()).await {
            Ok(monitors) => {
                tracing::info!(test = "test_clickhouse_logs", "OK");
                eprintln!("{monitors:#?}")
            }
            Err(err) => {
                tracing::error!(
                    test = "test_clickhouse_logs",
                    error = err.to_string(),
                    "failed to get"
                );
                panic!("{}", err);
            }
        };
    }
}
