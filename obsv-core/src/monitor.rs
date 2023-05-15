//! Monitors

use clickhouse_client::schema::DbRow;
use serde::{Deserialize, Serialize};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// A monitor
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "monitors"))]
pub struct Monitor {
    /// Friendly ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: String,
    /// Friendly name
    pub name: String,
    /// Kind (eg HTTP, GRPC)
    pub kind: String,
    /// Target service (eg service name, url)
    pub target: String,
}

/// A monitor check
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "monitor_checks"))]
pub struct MonitorCheck {
    /// ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: u128,
    /// Monitor ID
    pub monitor_id: String,
    /// Timestamp (ns FROM EPOCH)
    pub timestamp: u64,
    /// Error (the error is defined if there is an error)
    pub error: Option<String>,
    /// Response time (in ns)
    pub resp_time_ns: Option<u64>,
}
