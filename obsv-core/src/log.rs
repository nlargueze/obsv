//! Logs

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::{Attr, Attrs};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// Logs collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logs(Vec<Log>);

impl std::fmt::Display for Logs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, log) in self.0.iter().enumerate() {
            write!(f, "{log}")?;
            if i < self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Logs {
    /// Creates a new collection
    pub fn new(logs: Vec<Log>) -> Self {
        Self(logs)
    }
}

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

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = OffsetDateTime::from_unix_timestamp_nanos(self.timestamp as i128)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH);

        write!(
            f,
            "[{}] [{}] {} || {}",
            dt.format(&Rfc3339).unwrap(),
            self.id,
            self.message,
            self.attrs,
        )
    }
}

impl Log {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }

    /// Adds attributes
    pub fn add_attrs(&mut self, attrs: impl IntoIterator<Item = impl Into<Attr>>) -> &mut Self {
        for attr in attrs.into_iter() {
            self.attrs.push(attr.into());
        }
        self
    }
}
