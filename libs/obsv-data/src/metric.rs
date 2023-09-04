//! Metrics

use serde::{Deserialize, Serialize};

use crate::shared::Value;

/// A metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Name
    pub name: String,
    /// Description
    pub descr: String,
    /// Unit description
    pub unit: String,
    /// Value
    pub value: Value,
}
