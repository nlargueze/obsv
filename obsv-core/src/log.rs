//! Logs

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::Attrs;

/// A collection of logs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Logs(pub Vec<Log>);

impl Logs {
    /// Creates a new collection
    pub fn new(logs: Vec<Log>) -> Self {
        Self(logs)
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

/// A log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// Resource name
    pub resource: String,
    /// Resource attributes
    pub resource_attrs: Attrs,
    /// Scope name
    pub scope: String,
    /// Resource attributes
    pub scope_attrs: Attrs,
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Level (severity)
    pub level: i32,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
    /// Trace id
    pub trace_id: u128,
    /// Span id
    pub span_id: u64,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attrs_str = self
            .attrs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "[{}] ({}) {} | {}",
            self.timestamp.format(&Rfc3339).expect("invalid datetime"),
            self.level,
            self.message,
            attrs_str
        )
    }
}
