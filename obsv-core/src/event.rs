//! Events

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::{Attr, Attrs};

/// User event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEvent {
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Kind
    pub kind: String,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl UserEvent {
    /// Creates a new user event
    pub fn new(kind: &str, message: &str) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc(),
            kind: kind.to_string(),
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
    pub fn add_attrs(&mut self, mut attrs: Vec<Attr>) -> &mut Self {
        self.attrs.append(&mut attrs);
        self
    }
}

impl std::fmt::Display for UserEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] ({}) {} || {}",
            self.timestamp.format(&Rfc3339).unwrap(),
            self.kind,
            self.message,
            self.attrs,
        )
    }
}
