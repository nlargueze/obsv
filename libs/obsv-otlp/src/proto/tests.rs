//! Tests

use super::{
    collector::trace::v1::ExportTraceServiceRequest,
    common::v1::{any_value::Value, AnyValue, InstrumentationScope, KeyValue},
    resource::v1::Resource,
    trace::v1::{span::Event, ResourceSpans, ScopeSpans, Span},
};
use prost::Message;

/// A test to encode and decode a
#[test]
fn protobuf_serde() {
    let value = ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(Resource {
                attributes: vec![KeyValue {
                    key: "service.name".to_string(),
                    value: Some(AnyValue {
                        value: Some(Value::StringValue("my_service".to_string())),
                    }),
                }],
                dropped_attributes_count: 0,
            }),
            schema_url: "schema_url".to_string(),
            scope_spans: vec![ScopeSpans {
                scope: Some(InstrumentationScope {
                    name: "my_span".to_string(),
                    version: "v1".to_string(),
                    attributes: vec![KeyValue {
                        key: "scoped_span.attr".to_string(),
                        value: Some(AnyValue {
                            value: Some(Value::StringValue("my_service".to_string())),
                        }),
                    }],
                    dropped_attributes_count: 0,
                }),
                spans: vec![Span {
                    // 16 bytes array
                    // there is an error, expecting a string with the hex value with no leading 0x
                    trace_id: 1_u128.to_le_bytes().to_vec(),
                    // 8 bytes array
                    span_id: 1_u64.to_le_bytes().to_vec(),
                    //
                    trace_state: "state".to_string(),
                    // [] not accepted
                    parent_span_id: vec![],
                    name: "my_span".to_string(),
                    kind: 1,
                    start_time_unix_nano: 1,
                    end_time_unix_nano: 2,
                    attributes: vec![KeyValue {
                        key: "attr1".to_string(),
                        value: Some(AnyValue {
                            value: Some(Value::StringValue("attr1_value".to_string())),
                        }),
                    }],
                    dropped_attributes_count: 0,
                    events: vec![Event {
                        time_unix_nano: 1,
                        name: "event 1".to_string(),
                        attributes: vec![],
                        dropped_attributes_count: 0,
                    }],
                    dropped_events_count: 0,
                    links: vec![],
                    dropped_links_count: 0,
                    // null not accepted
                    status: None,
                }],
                schema_url: "span.schema_url".to_string(),
            }],
        }],
    };

    let mut buf = Vec::new();
    value.encode(&mut buf).unwrap();
    eprintln!("{buf:?}");

    let decoded = ExportTraceServiceRequest::decode(&buf as &[u8]).unwrap();
    eprintln!("{decoded:#?}");
    assert_eq!(decoded, value);
}
