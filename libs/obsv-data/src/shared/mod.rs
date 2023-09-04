//! Shared structures

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    /// ID
    pub id: u128,
}

/// A service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: HashMap<String, Value>,
}

/// A span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// ID
    pub id: u64,
    /// Parent ID
    pub parent_id: Option<u64>,
    /// Name
    pub name: String,
    /// Start time (UNIX nanoseconds)
    pub start: i128,
    /// End time (UNIX nanoseconds)
    pub end: i128,
    /// Attributes
    pub attrs: HashMap<String, Value>,
    /// Events
    pub events: Vec<SpanEvent>,
}

/// A span event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    /// Timestamp (UNIX nanoseconds)
    pub timestamp: i128,
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: HashMap<String, Value>,
}

/// An attribute value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    None,
    Bool(bool),
    Uint(u64),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}
