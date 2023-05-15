//! Trace
//!
//! Grafana dashboard provides a TraceView chart which accepts traces in a specific format.
//!
//! Refer to:
//! - https://grafana.com/docs/grafana/latest/explore/trace-integration/
//! - https://github.com/grafana/grafana/blob/main/packages/grafana-data/src/types/trace.ts#L28

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Trace formate for Grafana
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    /// Trace ID
    pub trace_id: String,
    /// Span ID
    pub span_id: String,
    /// Parent span ID (or 'undefined')
    pub parent_span_id: Option<String>,
    /// Operation name
    pub operation_name: String,
    /// Service name
    pub service_name: String,
    /// Service tags
    pub service_tags: Vec<TraceKeyValuePair>,
    /// Start time (ms from EPOCH)
    pub start_time: u64,
    /// Duration (ms)
    pub duration: u64,
    // Logs
    pub logs: Vec<TraceLog>,
    // Reference
    pub references: Option<Vec<TraceSpanReference>>,
    /// Tags
    pub tags: Option<Vec<TraceKeyValuePair>>,
    /// Warnings
    pub warnings: Option<Vec<String>>,
    /// Stack traces
    pub stack_traces: Option<Vec<String>>,
}

/// Key-value pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceKeyValuePair {
    /// Key
    pub key: String,
    /// Value
    pub value: String,
}

/// Any value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnyValue {
    Null,
    Bool(bool),
    Int(isize),
    Uint(usize),
    String(String),
    Array(Vec<AnyValue>),
    Object(HashMap<String, AnyValue>),
}

/// TraceLog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLog {
    /// Timestamp (ms from EPOCH)
    pub timestamp: u64,
    /// Fields
    pub fields: Vec<TraceKeyValuePair>,
}

/// Trace span reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSpanReference {
    /// Trace ID
    pub trace_id: String,
    /// Span ID
    pub span_id: String,
    /// Tags
    pub tags: Option<Vec<TraceKeyValuePair>>,
}
