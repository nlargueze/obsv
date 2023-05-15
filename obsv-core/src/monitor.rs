//! Monitors

use clickhouse_client::schema::DbRow;
use serde::{Deserialize, Serialize};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// A monitor is the result of
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "monitors"))]
pub struct Monitor {
    /// ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: u128,
    /// Monitor ID
    pub monitor_id: String,
    /// Monitor (friendly) name
    pub monitor_name: String,
    /// Timestamp (ns FROM EPOCH)
    pub timestamp: u64,
    /// Error (the error is defined if there is an error)
    pub error: Option<String>,
    /// Response time (in ns)
    pub resp_time_ns: Option<u64>,
}
