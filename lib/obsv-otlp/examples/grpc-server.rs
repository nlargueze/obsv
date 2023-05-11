//! GRPC server

use obsv_otlp::proto::collector::trace::v1::{
    trace_service_server, ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use std::net::SocketAddr;
use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:4317".parse::<SocketAddr>().unwrap();

    eprintln!("GRPC listening on http://{addr} ...");
    tonic::transport::Server::builder()
        .add_service(trace_service_server::TraceServiceServer::new(
            GrpcTraceService::new(),
        ))
        .serve(addr)
        .await
        .unwrap();
}

/// GRPC servoce
#[derive(Clone)]
struct GrpcTraceService;

impl GrpcTraceService {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl trace_service_server::TraceService for GrpcTraceService {
    async fn export(
        &self,
        req: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        eprintln!("{req:#?}");
        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}
