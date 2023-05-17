//! Trace

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::Attrs;

/// A collection of spans
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Spans(Vec<Span>);

impl Spans {
    /// Creates a new collection of spans
    pub fn new(spans: Vec<Span>) -> Self {
        Self(spans)
    }
}

impl Deref for Spans {
    type Target = Vec<Span>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Spans {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Resource name
    pub resource: String,
    /// Resource attributes
    pub resource_attrs: Attrs,
    /// Scope name
    pub scope: String,
    /// Resource attributes
    pub scope_attrs: Attrs,
    /// Trace ID
    pub trace_id: u128,
    /// Span ID
    pub span_id: u64,
    /// Parent span ID (0 if no parent)
    pub parent_span_id: u64,
    /// Span name
    pub name: String,
    /// Kind
    pub kind: i32,
    /// Start time
    pub start: OffsetDateTime,
    /// End time
    pub end: OffsetDateTime,
    /// Span attributes
    pub attrs: Attrs,
    /// Events
    pub events: Vec<SpanEvent>,
    // TODO: add span links + span status
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attrs_str = self
            .attrs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "trace_id={}, id={}, parent_id={}, name={}, start={}, end={} | {}",
            self.trace_id,
            self.span_id,
            self.parent_span_id,
            self.name,
            self.start.format(&Rfc3339).unwrap(),
            self.end.format(&Rfc3339).unwrap(),
            attrs_str,
        )
    }
}

/// A span event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: Attrs,
}

impl std::fmt::Display for SpanEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attrs_str = self
            .attrs
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "[{}] {} || {}",
            self.timestamp.format(&Rfc3339).unwrap_or_default(),
            self.name,
            attrs_str,
        )
    }
}
