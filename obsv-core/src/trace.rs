//! Trace

use serde::{Deserialize, Serialize};

use crate::{attr::Attr, event::Event};

/// Spans collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spans(Vec<Span>);

impl Spans {
    /// Creates a new span collection
    pub fn new(spans: Vec<Span>) -> Self {
        Self(spans)
    }
}

/// A span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Trace ID
    pub trace_id: u128,
    /// Span ID
    pub id: u64,
    /// Parent span ID (0 if no parent)
    pub parent_id: u64,
    /// Span name
    pub name: String,
    /// Start time (ns from EPOCH)
    pub start: u64,
    /// End time (ns from EPOCH)
    pub end: u64,
    /// Attributes
    pub attrs: Vec<Attr>,
    /// Events
    pub events: Vec<Event>,
}

impl Span {
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
