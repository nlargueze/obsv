//! Trace

use serde::{Deserialize, Serialize};

use crate::{
    attr::{Attr, Attrs},
    event::{Event, Events},
};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;

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
    pub events: Events,
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
    pub fn add_event(&mut self, event: impl Into<Event>) -> &mut Self {
        self.events.push(event.into());
        self
    }

    /// Adds events
    pub fn add_events(&mut self, events: impl IntoIterator<Item = impl Into<Event>>) -> &mut Self {
        for event in events.into_iter() {
            self.events.push(event.into());
        }
        self
    }
}
