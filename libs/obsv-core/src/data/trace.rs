//! Traces

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{AttrValue, Scope, Service};

/// A collection of trace data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceData {
    /// Spans
    pub spans: Vec<ServiceSpans>,
}

/// A collection of spans for a specific service and scope
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpans {
    /// Service
    pub service: Service,
    /// Scope
    pub scope: Option<Scope>,
    /// Spans
    pub spans: Vec<Span>,
}

/// A trace span
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Span {
    /// Span ID
    pub id: u64,
    /// Parent span ID
    pub parent_id: Option<u64>,
    /// Trace ID
    pub trace_id: u128,
    /// Span name
    pub name: String,
    /// Start time (UNIX nanoseconds)
    pub start: i128,
    /// End time (UNIX nanoseconds)
    pub end: i128,
    /// Span attributes
    pub attrs: HashMap<String, AttrValue>,
    /// Span events
    pub events: Vec<SpanEvent>,
    // service name + attrs
    // scope name + attrs
    // logs
}

/// A span event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpanEvent {
    /// Timestamp (UNIX nanoseconds)
    pub timestamp: i128,
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: HashMap<String, AttrValue>,
}
