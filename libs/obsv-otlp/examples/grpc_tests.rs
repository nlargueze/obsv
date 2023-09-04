//! Send tests to the GRPC endpoint

use obsv_otlp::proto::collector::trace::v1::{
    trace_service_client::TraceServiceClient, ExportTraceServiceRequest,
};
use tonic::Request;

/// Trace data
static TRACE_DATA: &str = include_str!("trace.json");

#[tokio::main]
async fn main() {
    let url = "http://0.0.0.0:4317";
    let mut client = TraceServiceClient::connect(url).await.unwrap();

    let trace_request = serde_json::from_str::<ExportTraceServiceRequest>(TRACE_DATA).unwrap();
    for i in 0..10 {
        eprintln!("Running test {i}");
        let res = client
            .export(Request::new(trace_request.clone()))
            .await
            .unwrap();
        eprintln!("{:?}", res);
    }
}
