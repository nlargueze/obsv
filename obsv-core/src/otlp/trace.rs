//! OTLP traces

use std::collections::HashMap;

use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A trace is a collection of spans.
pub struct Trace {
    pub spans: Vec<Span>,
}

/// Span
///
/// A span is a unit operation of work.
#[derive(Debug, Serialize)]
pub struct Span {
    /// Name
    pub name: String,
    /// Context
    pub context: SpanContext,
    /// Parent ID
    ///
    /// None for root spans
    pub parent_id: Option<u64>,
    /// Start time
    pub start_time: OffsetDateTime,
    /// End time
    pub end_time: OffsetDateTime,
    /// Key-value pairs of attributes
    ///
    /// - Keys are non null string values
    /// - Values must be a non-null string, boolean, floating point value, integer, or an array of these values
    pub attributes: HashMap<String, Box<dyn AttrValue>>,
    /// Events within the span
    pub events: Vec<SpanEvent>,
    /// Kind,
    pub span_kind: SpanKind,
    /// A link is used to associate 1 or more related spans.
    ///
    /// Eg. async operations
    pub links: Vec<Span>,
    /// Span status
    pub status: SpanStatus,
    /// TBD
    pub resource: String,
    // /// TBD
    // pub instrumentation_lib: InstrumentationLibrary,
}

/// Attribute value
pub trait AttrValue: std::fmt::Debug + erased_serde::Serialize {}

serialize_trait_object!(AttrValue);

impl AttrValue for String {}
impl AttrValue for bool {}
impl AttrValue for u8 {}
impl AttrValue for u16 {}
impl AttrValue for u32 {}
impl AttrValue for u64 {}
impl AttrValue for u128 {}
impl AttrValue for usize {}
impl AttrValue for i8 {}
impl AttrValue for i16 {}
impl AttrValue for i32 {}
impl AttrValue for i64 {}
impl AttrValue for i128 {}
impl AttrValue for isize {}
impl AttrValue for f32 {}
impl AttrValue for f64 {}

impl<T> AttrValue for Vec<T> where T: AttrValue + Serialize {}

/// Span semantic attributes
///
/// See https://opentelemetry.io/docs/specification/otel/trace/semantic_conventions/
pub enum SpanSemAttributes {
    NetTransport(String),
    // TODO: continue this list
}

impl SpanSemAttributes {
    pub fn key_value(&self) -> (String, Box<dyn AttrValue>) {
        match self {
            Self::NetTransport(v) => ("net.transport".to_string(), Box::new(v.to_string())),
        }
    }
}

impl From<SpanSemAttributes> for (String, Box<dyn AttrValue>) {
    fn from(value: SpanSemAttributes) -> Self {
        value.key_value()
    }
}

impl Span {
    /// Adds a semantic attribute
    pub fn add_attr(&mut self, attr: impl Into<(String, Box<dyn AttrValue>)>) {
        let (key, value) = attr.into();
        self.attributes.insert(key, value);
    }
}

/// Span context
//
/// The context is what is used to correlated traces together, and is the core concept
/// for trace propagation. The context can have different format but the default is the
/// [W3C TraceContext](https://www.w3.org/TR/trace-context/).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanContext {
    /// Trace ID
    ///
    /// This is used to correlate spans together
    ///
    /// "It must be a 16-byte array with at least one non-zero byte"
    pub trace_id: u128,
    /// Span ID
    ///
    /// This is the parent span if coming from another service for instance
    ///
    /// "A valid span identifier is an 8-byte array with at least one non-zero byte"
    pub span_id: u64,
    // Span flags: a binary encoding containing information about the trace
    // Trace State: key--value pair of vendor-specific trace information
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    pub name: String,
    pub timestamp: OffsetDateTime,
    pub attributes: HashMap<String, String>,
}

/// Span status
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum SpanStatus {
    Unset,
    Ok,
    Error,
}

/// Span kind
///
/// A kind is a hint on how the span should be assembled.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum SpanKind {
    /// Synchronous outgoing remote call (eg HTTP or DB request)
    Client,
    /// Synchronous incoming remote call (eg HTTP incoming request)
    Server,
    /// Span that do not cross service boundaries
    Internal,
    /// Async job creation
    Producer,
    /// Async job processing
    Consumer,
}
