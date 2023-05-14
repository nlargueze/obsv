//! Events

use serde::{Deserialize, Serialize};

use crate::attr::Attr;

/// An event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Kind
    pub kind: String,
    /// Name
    pub name: String,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Vec<Attr>,
}

impl Event {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }
}
