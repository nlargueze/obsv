//! Trace

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::{Attr, Attrs};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

/// Spans collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spans(Vec<Span>);

impl std::fmt::Display for Spans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, span) in self.0.iter().enumerate() {
            write!(f, "{span}")?;
            if i < self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Spans {
    /// Creates a new span collection
    pub fn new(spans: Vec<Span>) -> Self {
        Self(spans)
    }
}

/// A span
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "traces"))]
pub struct Span {
    /// Span ID
    pub id: u128,
    #[cfg_attr(feature = "clickhouse", db(primary))]
    /// Trace ID
    pub trace_id: u128,
    /// Parent span ID (0 if no parent)
    pub parent_id: u128,
    /// Span name
    pub name: String,
    /// Start time (ns from EPOCH)
    pub start: u64,
    /// End time (ns from EPOCH)
    pub end: u64,
    /// Attributes
    pub attrs: Attrs,
    /// Events
    pub events: SpanEvents,
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt_start = OffsetDateTime::from_unix_timestamp_nanos(self.start as i128)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH);
        let dt_end = OffsetDateTime::from_unix_timestamp_nanos(self.end as i128)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH);

        write!(
            f,
            "trace_id={}, id={}, parent_id={}, name={}, start={}, end={} || {}",
            self.trace_id,
            self.id,
            self.parent_id,
            self.name,
            dt_start.format(&Rfc3339).unwrap(),
            dt_end.format(&Rfc3339).unwrap(),
            self.attrs,
        )
    }
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

    /// Adds an event
    pub fn add_event(&mut self, event: impl Into<SpanEvent>) -> &mut Self {
        self.events.push(event.into());
        self
    }

    /// Adds events
    pub fn add_events(
        &mut self,
        events: impl IntoIterator<Item = impl Into<SpanEvent>>,
    ) -> &mut Self {
        for event in events.into_iter() {
            self.events.push(event.into());
        }
        self
    }
}

/// Collection of span events
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpanEvents(pub Vec<SpanEvent>);

impl SpanEvents {
    /// Create a new [Events]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an event
    pub fn push(&mut self, event: SpanEvent) {
        self.0.push(event);
    }
}

impl From<Vec<SpanEvent>> for SpanEvents {
    fn from(value: Vec<SpanEvent>) -> Self {
        Self(value)
    }
}

/// A span event
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpanEvent {
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Name
    pub name: String,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl SpanEvent {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }
}
