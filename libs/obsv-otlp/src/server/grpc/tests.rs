//! Tests

use crate::{
    proto::collector::trace::v1::{
        trace_service_client::TraceServiceClient, trace_service_server::TraceService,
        ExportTraceServiceRequest, ExportTraceServiceResponse,
    },
    server::grpc::GrpcServer,
};
use tokio::sync::oneshot;
use tonic::{Request, Response, Status};

/// Trace data
static TRACE_DATA: &str = include_str!("trace.json");

/// Tests the GRPC endpoint
#[tokio::test]
async fn grpc_server() {
    // signal
    let (tx, rx) = oneshot::channel();

    // run the server inside a task
    let run_server = tokio::spawn(async {
        let shutdown = async {
            rx.await.unwrap();
        };

        GrpcServer::new()
            .addr("127.0.0.1:4317")
            .shutdown(shutdown)
            .trace_service(MyTraceService)
            .start()
            .await
            .unwrap();
    });

    // run the client inside a task
    let run_tests = tokio::spawn(async {
        let mut client = TraceServiceClient::connect("http://localhost:4317")
            .await
            .unwrap();
        let trace_request = serde_json::from_str::<ExportTraceServiceRequest>(TRACE_DATA).unwrap();
        for i in 0..10 {
            eprintln!("Running test {i}");
            let _res = client
                .export(Request::new(trace_request.clone()))
                .await
                .unwrap();
            // println!("{:?}", res);
        }

        // send a signal to stop the server
        tx.send(()).unwrap();
    });

    tokio::try_join!(run_server, run_tests).unwrap();
}

/// Trace service implementation
struct MyTraceService;

#[tonic::async_trait]
impl TraceService for MyTraceService {
    async fn export(
        &self,
        _request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        // eprintln!("=> Received request");
        // eprintln!("{:?}", request);

        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}
