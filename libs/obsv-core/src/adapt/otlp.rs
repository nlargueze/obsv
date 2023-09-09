//! OpenTelemetry adapter

use obsv_otlp::proto::{
    collector::{
        logs::v1::ExportLogsServiceRequest, metrics::v1::ExportMetricsServiceRequest,
        trace::v1::ExportTraceServiceRequest,
    },
    common::v1::{any_value::Value, AnyValue, InstrumentationScope, KeyValue},
    logs::v1::{LogRecord, ResourceLogs, ScopeLogs},
    metrics::v1::{
        metric::Data, summary_data_point::ValueAtQuantile, Metric, ResourceMetrics, ScopeMetrics,
        Summary, SummaryDataPoint,
    },
    resource::v1::Resource,
    trace::v1::{
        span::{Event, Link},
        ResourceSpans, ScopeSpans, Span, Status,
    },
};

use crate::data::{ServiceSpans, TraceData};

impl From<ExportTraceServiceRequest> for TraceData {
    fn from(value: ExportTraceServiceRequest) -> Self {
        todo!()
    }
}

// fn test_trace() {
//     let _ = ExportTraceServiceRequest {
//         resource_spans: vec![ResourceSpans {
//             // a resource is the entity producing the telemetry
//             resource: Some(Resource {
//                 attributes: vec![KeyValue {
//                     key: "key".to_string(),
//                     value: Some(AnyValue {
//                         value: Some(Value::BoolValue(false)),
//                     }),
//                 }],
//                 dropped_attributes_count: 0,
//             }),
//             // a unit of scope is the unit of code (eg package) producing the telemetry
//             scope_spans: vec![ScopeSpans {
//                 scope: Some(InstrumentationScope {
//                     name: "my scope".to_string(),
//                     version: "v1".to_string(),
//                     attributes: vec![],
//                     dropped_attributes_count: 0,
//                 }),
//                 spans: vec![Span {
//                     trace_id: [0x00; 16].to_vec(),
//                     span_id: [0x00; 8].to_vec(),
//                     trace_state: "state".to_string(),
//                     parent_span_id: [0x00; 8].to_vec(),
//                     name: "span name".to_string(),
//                     kind: 1,
//                     start_time_unix_nano: 1,
//                     end_time_unix_nano: 2,
//                     attributes: vec![],
//                     dropped_attributes_count: 0,
//                     events: vec![Event {
//                         time_unix_nano: 1,
//                         name: "my_event".to_string(),
//                         attributes: vec![],
//                         dropped_attributes_count: 0,
//                     }],
//                     dropped_events_count: 0,
//                     links: vec![Link {
//                         trace_id: [0x00; 16].to_vec(),
//                         span_id: [0x00; 8].to_vec(),
//                         trace_state: "state".to_string(),
//                         attributes: vec![],
//                         dropped_attributes_count: 0,
//                     }],
//                     dropped_links_count: 0,
//                     status: Some(Status {
//                         message: "status message".to_string(),
//                         code: 1,
//                     }),
//                 }],
//                 schema_url: "scope_schema_url".to_string(),
//             }],
//             schema_url: "resource_schema_url".to_string(),
//         }],
//     };
// }

// fn test_logs() {
//     let _ = ExportLogsServiceRequest {
//         resource_logs: vec![ResourceLogs {
//             resource: Some(Resource {
//                 attributes: vec![KeyValue {
//                     key: "key".to_string(),
//                     value: Some(AnyValue {
//                         value: Some(Value::BoolValue(false)),
//                     }),
//                 }],
//                 dropped_attributes_count: 0,
//             }),
//             scope_logs: vec![ScopeLogs {
//                 scope: Some(InstrumentationScope {
//                     name: "my scope".to_string(),
//                     version: "v1".to_string(),
//                     attributes: vec![],
//                     dropped_attributes_count: 0,
//                 }),
//                 log_records: vec![LogRecord {
//                     time_unix_nano: 10_000_000,
//                     observed_time_unix_nano: 10_000_000,
//                     severity_number: 1,
//                     severity_text: "info".to_string(),
//                     body: Some(AnyValue {
//                         value: Some(Value::StringValue("body".to_string())),
//                     }),
//                     attributes: vec![],
//                     dropped_attributes_count: 0,
//                     flags: 0,
//                     trace_id: [0x00; 16].to_vec(),
//                     span_id: [0x00; 8].to_vec(),
//                 }],
//                 schema_url: "schema_url".to_string(),
//             }],
//             schema_url: "schema_url".to_string(),
//         }],
//     };
// }

// fn test_metrics() {
//     let _ = ExportMetricsServiceRequest {
//         resource_metrics: vec![ResourceMetrics {
//             resource: Some(Resource {
//                 attributes: vec![KeyValue {
//                     key: "key".to_string(),
//                     value: Some(AnyValue {
//                         value: Some(Value::BoolValue(false)),
//                     }),
//                 }],
//                 dropped_attributes_count: 0,
//             }),
//             scope_metrics: vec![ScopeMetrics {
//                 scope: Some(InstrumentationScope {
//                     name: "my scope".to_string(),
//                     version: "v1".to_string(),
//                     attributes: vec![],
//                     dropped_attributes_count: 0,
//                 }),
//                 metrics: vec![Metric {
//                     name: "my metric".to_string(),
//                     description: "desc".to_string(),
//                     unit: "MB".to_string(),
//                     data: Some(Data::Summary(Summary {
//                         data_points: vec![SummaryDataPoint {
//                             attributes: vec![],
//                             start_time_unix_nano: 0,
//                             time_unix_nano: 1,
//                             count: 1,
//                             sum: 2.0,
//                             quantile_values: vec![ValueAtQuantile {
//                                 quantile: 1.1,
//                                 value: 1.2,
//                             }],
//                             flags: 0,
//                         }],
//                     })),
//                 }],
//                 schema_url: "schema_url".to_string(),
//             }],
//             schema_url: "schema_url".to_string(),
//         }],
//     };
// }
