//! Grafana

use std::collections::HashMap;

use crate::data::AttrValue;

/// A Grafana data frame for the trace viewer chart
#[derive(Debug, Clone)]
pub struct GrafanaDataFrame {
    /// traceID (string)
    ///
    /// Identifier for the entire trace. There should be only one trace in the data frame.
    pub trace_id: String,
    /// spanID  (string)
    ///
    /// Identifier for the current span. SpanIDs should be unique per trace.
    pub span_id: String,
    /// parentSpanID (string)
    ///
    /// SpanID of the parent span to create child parent relationship in the trace view. Can be undefined for root span without a parent.
    pub parent_span_id: String,
    /// serviceName (string)
    ///
    /// Name of the service this span is part of.
    pub service_name: String,
    /// serviceTags TraceKeyValuePair[]
    ///
    /// List of tags relevant for the service.
    pub service_tags: HashMap<String, AttrValue>,
    /// startTime (number)
    ///
    /// Start time of the span in millisecond epoch time.
    pub start_time: f64,
    /// duration (number)
    ///
    /// Duration of the span in milliseconds.
    pub duration: f64,
    /// logs (TraceLog[]) - optional
    ///
    /// List of logs associated with the current span.
    pub logs: Option<Vec<GrafanaTraceLog>>,
    /// tags TraceKeyValuePair[] - optional
    ///
    /// List of tags associated with the current span.
    pub tags: Option<HashMap<String, AttrValue>>,
    /// warnings (string[]) - optional
    ///
    /// List of warnings associated with the current span.
    pub warnings: Option<Vec<String>>,
    /// stackTraces (string[]) - optional
    ///
    /// List of stack traces associated with the current span.
    pub stack_traces: Option<Vec<String>>,
    /// errorIconColor  (string) - optional
    ///
    /// Color of the error icon in case span is tagged with error: true.
    pub error_icon_color: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GrafanaTraceLog {
    /// Millisecond epoch time
    pub timestamp: f64,
    /// Fields
    pub fields: HashMap<String, AttrValue>,
}
