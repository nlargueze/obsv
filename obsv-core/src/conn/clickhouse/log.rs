//! DB logs

use clickhouse_client::orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::attr::Attrs;

/// A DB log
#[derive(Debug, Clone, Serialize, Deserialize, DbRecord)]
#[db(table = "logs")]
pub struct DbLog {
    /// ID
    #[db(primary)]
    pub id: u128,
    /// Trace id
    pub trace_id: u128,
    /// Span id
    pub span_id: u64,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Level (severity)
    pub level: i32,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl Default for DbLog {
    fn default() -> Self {
        Self {
            id: 0,
            timestamp: 0,
            message: String::new(),
            attrs: Attrs::new(),
            trace_id: 0,
            span_id: 0,
            level: 0,
        }
    }
}

impl From<crate::Log> for DbLog {
    fn from(value: crate::Log) -> Self {
        Self {
            id: value.id,
            timestamp: value.timestamp,
            level: value.level,
            message: value.message,
            trace_id: value.trace_id,
            span_id: value.span_id,
            attrs: value.attrs,
        }
    }
}

impl From<DbLog> for crate::Log {
    fn from(value: DbLog) -> Self {
        Self {
            id: value.id,
            timestamp: value.timestamp,
            level: value.level,
            message: value.message,
            trace_id: value.trace_id,
            span_id: value.span_id,
            attrs: value.attrs,
        }
    }
}
