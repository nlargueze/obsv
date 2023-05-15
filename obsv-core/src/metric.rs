//! Metrics
//!
//! TODO

use serde::{Deserialize, Serialize};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// A metric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "metrics"))]
pub struct Metric {
    /// ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Value
    pub value: f64,
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            id: 0,
            timestamp: 0,
            value: 0.0,
        }
    }
}
