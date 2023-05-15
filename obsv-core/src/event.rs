//! Events

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::{Attr, Attrs};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// An event
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "events"))]
pub struct Event {
    /// ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Kind
    pub kind: String,
    /// Name
    pub name: String,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = OffsetDateTime::from_unix_timestamp_nanos(self.timestamp as i128)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH);

        write!(
            f,
            "[{}] [{}] ({}) {} || {}",
            dt.format(&Rfc3339).unwrap(),
            self.id,
            self.kind,
            self.message,
            self.attrs,
        )
    }
}

impl Event {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }
}
