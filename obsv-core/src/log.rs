//! Logs

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::attr::Attr;

/// A log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// ID
    pub id: u128,
    /// Date
    pub timestamp: OffsetDateTime,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Vec<Attr>,
}

impl Log {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }
}
