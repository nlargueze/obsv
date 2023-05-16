//! DB logs

use clickhouse_client::schema::prelude::*;
use serde::{Deserialize, Serialize};

use crate::attr::Attrs;

/// A DB log
#[derive(Debug, Clone, Serialize, Deserialize, DbRow)]
#[db(table = "logs")]
pub struct DbLog {
    /// ID
    #[db(primary)]
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
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
        }
    }
}

impl From<crate::Log> for DbLog {
    fn from(value: crate::Log) -> Self {
        Self
    }
}
