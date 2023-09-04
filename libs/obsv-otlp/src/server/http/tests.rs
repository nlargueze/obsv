//! Tests

use salvo::{http::ResBody, hyper::header::CONTENT_TYPE, prelude::*, test::TestClient};
use tokio::sync::oneshot;

use crate::{
    proto::collector::trace::v1::{ExportTraceServiceRequest, ExportTraceServiceResponse},
    server::http::HttpConvert,
};

use super::{APPLICATION_PROTOBUF, ENDPOINT_LOGS, ENDPOINT_TRACES};

/// Trace data
static TRACE_DATA: &str = include_str!("trace.json");

#[tokio::test]
async fn http_server() {
    let (tx, rx) = oneshot::channel::<()>();

    // run the server inside a task
    let run_server = tokio::spawn(async {
        let acceptor = TcpListener::new("127.0.0.1:4318").bind().await;

        let shutdown = async {
            rx.await.unwrap();
        };

        eprintln!("Server started on 127.0.0.1::4318");
        Server::new(acceptor)
            .serve_with_graceful_shutdown(router(), shutdown, None)
            .await;
    });

    // run the client inside a task
    let run_tests = tokio::spawn(async {
        let trace_request = serde_json::from_str::<ExportTraceServiceRequest>(TRACE_DATA).unwrap();
        let content_type = APPLICATION_PROTOBUF;
        let trace_request_bytes = trace_request.into_http_body(content_type).unwrap();

        for i in 0..10 {
            eprintln!("Running test {i}");
            let res = TestClient::post("http://127.0.0.1:4318/v1/traces")
                .add_header(CONTENT_TYPE, content_type, true)
                .bytes(trace_request_bytes.clone())
                .send(router())
                .await;
            if let Some(code) = res.status_code {
                if !code.is_success() {
                    panic!("Failed request");
                }
            }
        }

        // send a signal to stop the server
        tx.send(()).unwrap();
    });

    tokio::try_join!(run_server, run_tests).unwrap();
}

/// Creates the router
fn router() -> Router {
    Router::new()
        .push(Router::with_path("").get(hello))
        .push(Router::with_path(ENDPOINT_TRACES).post(handle_traces))
        .push(Router::with_path(ENDPOINT_LOGS).post(handle_logs))
}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

/// Handler for traces
#[handler]
async fn handle_traces(req: &mut Request, res: &mut Response) {
    eprintln!("Received trace request");
    // eprintln!("{req:?}");

    const PAYLOAD_MAX_SIZE: usize = 52_428_800; // 50MB
    let content_type = req.header::<String>(CONTENT_TYPE).unwrap_or_default();
    let body = req.payload_with_max_size(PAYLOAD_MAX_SIZE).await.unwrap();
    let _otlp_req = match ExportTraceServiceRequest::from_http_request(&content_type, body) {
        Ok(r) => r,
        Err(err) => {
            res.status_code(StatusCode::BAD_REQUEST)
                .render(format!("Cannot parse requets body: {err}"));
            return;
        }
    };

    let otlp_res = ExportTraceServiceResponse {
        partial_success: None,
    };

    let body = otlp_res.into_http_body(&content_type).unwrap();
    res.status_code(StatusCode::OK)
        .add_header(CONTENT_TYPE, content_type, true)
        .unwrap()
        .body(ResBody::Once(body.into()));
}

#[handler]
async fn handle_logs() {
    unimplemented!("handle_logs")
}
