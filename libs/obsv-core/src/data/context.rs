//! Context

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::http::{TraceParentHeader, TraceStateHeader};

/// Context
///
/// A context is used to propagate traces across services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Trace ID
    pub trace_id: u128,
    /// Parent span ID
    pub span_id: u64,
    /// Flags
    pub flags: u8,
    /// State
    pub state: HashMap<String, String>,
}

impl Context {
    /// Returns the HTTP header `traceparent`
    pub fn parent_header(&self) -> TraceParentHeader {
        TraceParentHeader {
            version: 0,
            trace_id: self.trace_id,
            parent_id: self.span_id,
            flags: self.flags,
        }
    }

    /// Returns the HTTP header `tracestate`
    pub fn state_header(&self) -> TraceStateHeader {
        TraceStateHeader {
            values: self.state.clone(),
        }
    }
}
