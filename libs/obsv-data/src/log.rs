//! Logs

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::{Service, Span, Trace, Value};

/// A log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// Trace
    pub trace: Trace,
    /// Service
    pub service: Service,
    /// Span
    pub span: Span,
    /// Timestamp (UNIX nanoseconds)
    pub timestamp: i128,
    /// Level (severity)
    pub level: u8,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: HashMap<String, Value>,
}
