//! Metrics

use clickhouse_client::orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::attr::Attrs;

/// A DB metric
#[derive(Debug, Clone, Serialize, Deserialize, DbRecord)]
#[db(table = "metrics")]
pub struct DbMetric {
    /// ID
    #[db(primary)]
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: Attrs,
}

impl Default for DbMetric {
    fn default() -> Self {
        Self {
            id: 0,
            timestamp: 0,
            name: String::new(),
            attrs: Attrs::new(),
        }
    }
}

impl From<crate::Metric> for DbMetric {
    fn from(value: crate::Metric) -> Self {
        Self {
            id: value.id,
            timestamp: value.timestamp,
            name: value.name,
            attrs: value.attrs,
        }
    }
}

impl From<DbMetric> for crate::Metric {
    fn from(value: DbMetric) -> Self {
        Self {
            id: value.id,
            timestamp: value.timestamp,
            name: value.name,
            attrs: value.attrs,
        }
    }
}
