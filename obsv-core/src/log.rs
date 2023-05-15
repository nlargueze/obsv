//! Logs

use serde::{Deserialize, Serialize};

use crate::attr::{Attr, Attrs};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// A log
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "logs"))]
pub struct Log {
    /// ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            id: 0,
            timestamp: 0,
            message: String::new(),
            attrs: Attrs::new(),
        }
    }
}

impl Log {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }
}
