//! Logs

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{AttrValue, Scope, Service};

/// A set of log data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogData {
    /// Logs
    pub logs: Vec<ServiceLogs>,
}

/// A collection of logs for a specific service and scope
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceLogs {
    /// Service
    pub service: Service,
    /// Scope
    pub scope: Option<Scope>,
    /// Logs
    pub logs: Vec<Log>,
}

/// A single log
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    /// Trace ID
    pub trace_id: u128,
    /// Span ID
    pub span_id: u64,
    /// Timestamp (UNIX nanoseconds)
    pub timestamp: i128,
    /// Level (severity)
    pub level: i16,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: HashMap<String, AttrValue>,
}
