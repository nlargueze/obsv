//! Clickhouse DB client

use std::collections::HashMap;

use clickhouse_client::orm::prelude::*;

use crate::data::AttrValue;

use super::DbClient;

// Clickhouse DB client
#[derive(Debug)]
pub struct ChClient {}

impl DbClient for ChClient {}

/// A span in Clickhouse DB
#[derive(Debug, AsChRecord)]
#[ch(table = "spans")]
pub struct ChSpan {
    /// Span ID
    #[ch(primary_key)]
    pub id: u64,
    /// Trace ID
    pub trace_id: u128,
    /// Parent span ID
    pub parent_span_id: Option<String>,
    /// Service
    pub service: String,
    /// Service attributes
    pub service_attrs: HashMap<String, AttrValue>,
    /// Scope
    pub scope: String,
    /// Scope attributes
    pub scope_attrs: HashMap<String, AttrValue>,
    /// Span name
    pub name: String,
    /// Start time (UNIX nanoseconds)
    pub start: i128,
    /// End time (UNIX nanoseconds)
    pub end: i128,
    /// Span attributes
    pub attrs: HashMap<String, AttrValue>,
}

/// A span event in Clickhouse DB
#[derive(Debug, AsChRecord)]
#[ch(table = "spans_events")]
pub struct ChSpanEvent {
    /// Event ID
    #[ch(primary_key)]
    pub id: u128,
    /// Span ID
    pub span_id: u64,
    /// Timestamp (UNIX nanoseconds)
    pub timestamp: i128,
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: HashMap<String, AttrValue>,
}

impl ChValue for AttrValue {
    fn ch_type() -> Type {
        todo!()
    }

    fn into_ch_value(self) -> Value {
        todo!()
    }

    fn from_ch_value(value: Value) -> Result<Self, Error> {
        todo!()
    }
}
