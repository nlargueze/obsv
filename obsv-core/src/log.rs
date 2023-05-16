//! Logs

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::attr::{Attr, Attrs};

/// A log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// ID
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl Log {
    /// Creates a new log
    pub fn new(message: &str) -> Self {
        Self {
            id: Uuid::new_v4().as_u128(),
            timestamp: OffsetDateTime::now_utc().unix_timestamp_nanos() as u64,
            message: message.to_string(),
            attrs: Attrs::new(),
        }
    }

    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }

    /// Adds attributes
    pub fn add_attrs(&mut self, attrs: impl IntoIterator<Item = impl Into<Attr>>) -> &mut Self {
        self.attrs
            .0
            .append(&mut attrs.into_iter().map(|a| a.into()).collect());
        self
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

/// A collection of logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logs(pub Vec<Log>);

impl Logs {
    /// Creates a new collection
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Deref for Logs {
    type Target = Vec<Log>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Logs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
