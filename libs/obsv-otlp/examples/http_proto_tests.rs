//! Send tests to the HTTP endpoint

use obsv_otlp::{
    proto::collector::trace::v1::{ExportTraceServiceRequest, ExportTraceServiceResponse},
    server::http::{HttpConvert, APPLICATION_PROTOBUF},
};
use reqwest::{header::CONTENT_TYPE, Client};

/// Trace data
static TRACE_DATA: &str = include_str!("trace.json");

#[tokio::main]
async fn main() {
    let client = Client::new();
    let url = "http://0.0.0.0:4318/v1/traces";
    let request = serde_json::from_str::<ExportTraceServiceRequest>(TRACE_DATA).unwrap();
    let body = request.into_http_body(APPLICATION_PROTOBUF).unwrap();

    for i in 0..10 {
        eprintln!("Running test #{i}");
        let res = client
            .post(url)
            .header(CONTENT_TYPE, APPLICATION_PROTOBUF)
            .body(body.clone())
            .send()
            .await
            .unwrap();
        let content_type = res
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let body = res.bytes().await.unwrap();
        let res = ExportTraceServiceResponse::from_http_request(&content_type, &body).unwrap();
        eprintln!("{:#?}", res);
    }
}
