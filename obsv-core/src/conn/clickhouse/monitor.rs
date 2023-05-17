//! DB monitor

use std::time::Duration;

use clickhouse_client::{orm::prelude::*, query::Where};
use duration_string::DurationString;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{attr::Attrs, Monitor, MonitorCheck, MonitorCheckStatus};

use super::Error;

/// A DB monitor
#[derive(Debug, Clone, Default, DbRecord)]
#[db(table = "monitors")]
pub struct DbMonitor {
    /// Unique ID
    #[db(primary)]
    pub id: String,
    /// Friendly name
    pub name: String,
    /// Kind (eg HTTP, DNS, etc..)
    pub kind: String,
    /// Target (eg. service, url, etc..)
    pub target: String,
    /// Frequency
    pub frequency: String,
    /// Attributes
    pub attrs: Attrs,
}

/// A DB monitor check
#[derive(Debug, Clone, DbRecord)]
#[db(table = "monitors_checks")]
pub struct DbMonitorCheck {
    /// ID
    #[db(primary)]
    pub id: Uuid,
    /// Monitor ID
    pub monitor_id: String,
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Status (-1 = INIT, 0 = OK, 1 = Error)
    pub status: i8,
    /// Response time (in ns)
    pub resp_time: Option<u128>,
    /// Error message
    pub error: Option<String>,
}

impl Default for DbMonitorCheck {
    fn default() -> Self {
        Self {
            id: Default::default(),
            monitor_id: Default::default(),
            timestamp: OffsetDateTime::UNIX_EPOCH,
            status: Default::default(),
            resp_time: Default::default(),
            error: Default::default(),
        }
    }
}

impl From<Monitor> for DbMonitor {
    fn from(value: Monitor) -> Self {
        Self {
            id: value.id,
            name: value.name,
            kind: value.kind,
            target: value.target,
            frequency: DurationString::new(value.frequency).to_string(),
            attrs: Attrs::new(),
        }
    }
}

impl From<DbMonitor> for Monitor {
    fn from(value: DbMonitor) -> Self {
        Self {
            id: value.id,
            name: value.name,
            kind: value.kind,
            target: value.target,
            frequency: DurationString::from_string(value.frequency)
                .map(|dur_str| dur_str.into())
                .unwrap_or(Duration::ZERO),
        }
    }
}

impl From<MonitorCheck> for DbMonitorCheck {
    fn from(value: MonitorCheck) -> Self {
        let (status, resp_time, error) = match value.status {
            MonitorCheckStatus::Init => (-1, None, None),
            MonitorCheckStatus::Success { resp_time } => (0, Some(resp_time.as_nanos()), None),
            MonitorCheckStatus::Failure { resp_time, error } => {
                (1, Some(resp_time.as_nanos()), Some(error))
            }
        };
        Self {
            id: Uuid::new_v4(),
            monitor_id: value.monitor_id,
            timestamp: value.timestamp,
            status,
            resp_time,
            error,
        }
    }
}

impl From<DbMonitorCheck> for MonitorCheck {
    fn from(value: DbMonitorCheck) -> Self {
        let status = if value.status == -1 {
            let resp_time = Duration::from_nanos(value.resp_time.unwrap_or_default() as u64);
            let error = value.error.unwrap_or_default();
            MonitorCheckStatus::Failure { resp_time, error }
        } else if value.status == 0 {
            let resp_time = Duration::from_nanos(value.resp_time.unwrap_or_default() as u64);
            MonitorCheckStatus::Success { resp_time }
        } else {
            MonitorCheckStatus::Init
        };
        Self {
            monitor_id: value.monitor_id,
            timestamp: value.timestamp,
            status,
        }
    }
}

impl super::ClickhouseConnector {
    /// Inserts monitors
    pub async fn insert_monitors(&self, monitors: &[Monitor]) -> Result<(), Error> {
        let db_monitors = monitors
            .iter()
            .map(|m| DbMonitor::from(m.clone()))
            .collect::<Vec<_>>();
        Ok(self.client.insert(&db_monitors).await?)
    }

    /// Returns all the monitors
    pub async fn get_monitors(&self, where_cond: Where) -> Result<Vec<Monitor>, Error> {
        let db_monitors = self.client.select::<DbMonitor>(&[], where_cond).await?;
        Ok(db_monitors
            .into_iter()
            .map(Monitor::from)
            .collect::<Vec<_>>())
    }

    /// Inserts monitors checks
    pub async fn insert_monitors_checks(&self, checks: &[MonitorCheck]) -> Result<(), Error> {
        let db_checks = checks
            .iter()
            .map(|m| DbMonitorCheck::from(m.clone()))
            .collect::<Vec<_>>();
        Ok(self.client.insert(&db_checks).await?)
    }

    /// Returns the monitors checks
    pub async fn get_monitors_checks(&self, where_cond: Where) -> Result<Vec<MonitorCheck>, Error> {
        let db_checks = self
            .client
            .select::<DbMonitorCheck>(&[], where_cond)
            .await?;
        Ok(db_checks
            .into_iter()
            .map(MonitorCheck::from)
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Monitor;

    #[tokio::test]
    async fn test_clickhouse_monitors() {
        let conn = crate::conn::clickhouse::tests::init_clickhouse().await;
        let monitor = Monitor {
            id: "test_monitor".to_string(),
            name: "Test Monitor".to_string(),
            kind: "Kind".to_string(),
            target: "http://myapi.com".to_string(),
            frequency: Duration::from_secs(60),
        };

        if let Err(err) = conn.insert_monitors(&[monitor]).await {
            tracing::error!(
                test = "test_clickhouse_monitors",
                error = err.to_string(),
                "failed to insert"
            );
            panic!("{}", err);
        }
        match conn.get_monitors(Where::empty()).await {
            Ok(monitors) => {
                tracing::info!(test = "test_clickhouse_monitors", "OK");
                eprintln!("{monitors:#?}")
            }
            Err(err) => {
                panic!("{}", err);
            }
        };
    }

    #[tokio::test]
    async fn test_clickhouse_checks() {
        let conn = crate::conn::clickhouse::tests::init_clickhouse().await;
        let monitor = Monitor {
            id: "test_monitor".to_string(),
            name: "Test Monitor".to_string(),
            kind: "Kind".to_string(),
            target: "http://myapi.com".to_string(),
            frequency: Duration::from_secs(60),
        };

        let mut checks = vec![];
        for i in 0..10 {
            let mut check = monitor.start_check();
            if i % 2 == 0 {
                check.succeeded();
            } else {
                check.failed("some error");
            }
            checks.push(check);
        }

        if let Err(err) = conn.insert_monitors_checks(&checks).await {
            tracing::error!(
                test = "test_clickhouse_checks",
                error = err.to_string(),
                "failed to insert"
            );
            panic!("{}", err);
        }
        match conn.get_monitors_checks(Where::empty()).await {
            Ok(checks) => {
                tracing::info!(test = "test_clickhouse_checks", "OK");
                eprintln!("{checks:#?}")
            }
            Err(err) => {
                tracing::error!(
                    test = "test_clickhouse_checks",
                    error = err.to_string(),
                    "failed to select"
                );
                panic!("{}", err);
            }
        };
    }
}
