use hyper::{Client, Request};
use obsv_otlp::proto::{
    collector::trace::v1::ExportTraceServiceRequest,
    common::v1::{any_value::Value, AnyValue, InstrumentationScope, KeyValue},
    resource::v1::Resource,
    trace::v1::{span::Event, ResourceSpans, ScopeSpans, Span},
};

/// Tests the HTTP endpoint
#[tokio::test]
async fn test_http_trace() {
    let client = Client::builder().http2_only(false).build_http();

    let req_body = ExportTraceServiceRequest {
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
                    trace_id: 1_u128.to_be_bytes().to_vec(),
                    // 8 bytes array
                    span_id: 1_u64.to_be_bytes().to_vec(),
                    //
                    trace_state: "state".to_string(),
                    // [] not accepted
                    parent_span_id: 0_u64.to_be_bytes().to_vec(),
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
    let body = serde_json::to_string(&req_body).unwrap();
    eprintln!("{body}");

    let req = Request::builder()
        .method("POST")
        .uri("http://localhost:4318/v1/traces")
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap();

    let res = client.request(req).await.unwrap();
    if !res.status().is_success() {
        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        eprintln!("Error: {body_str}");
        panic!();
    }
}
