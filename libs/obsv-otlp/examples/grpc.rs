//! GRPC server

// use obsv_otlp::proto::collector::{
//     logs::v1::{
//         logs_service_server::{LogsService, LogsServiceServer},
//         ExportLogsServiceRequest, ExportLogsServiceResponse,
//     },
//     trace::v1::{
//         trace_service_server::{TraceService, TraceServiceServer},
//         ExportTraceServiceRequest, ExportTraceServiceResponse,
//     },
// };
// use std::net::SocketAddr;
// use tonic::{Request, Response, Status};

#[tokio::main]
async fn main() {
    // let addr = "127.0.0.1:4317".parse::<SocketAddr>().unwrap();

    // eprintln!();
    // eprintln!("GRPC listening on http://{addr} ...");
    // tonic::transport::Server::builder()
    //     .add_service(TraceServiceServer::new(GrpcHandler))
    //     .add_service(LogsServiceServer::new(GrpcHandler))
    //     .serve(addr)
    //     .await
    //     .unwrap();
}

// /// GRPC handler
// #[derive(Debug)]
// struct GrpcHandler;

// #[async_trait]
// impl OtlpHandler<ExportTraceServiceRequest, ExportTraceServiceResponse> for GrpcHandler {
//     async fn handle(
//         &self,
//         req: ExportTraceServiceRequest,
//     ) -> Result<ExportTraceServiceResponse, Error> {
//         eprintln!("{req:#?}");
//         Ok(ExportTraceServiceResponse {
//             partial_success: None,
//         })
//     }
// }

// #[async_trait]
// impl OtlpHandler<ExportLogsServiceRequest, ExportLogsServiceResponse> for GrpcHandler {
//     async fn handle(
//         &self,
//         req: ExportLogsServiceRequest,
//     ) -> Result<ExportLogsServiceResponse, Error> {
//         eprintln!("{req:#?}");
//         Ok(ExportLogsServiceResponse {
//             partial_success: None,
//         })
//     }
// }

// #[async_trait]
// impl TraceService for GrpcHandler {
//     async fn export(
//         &self,
//         req: Request<ExportTraceServiceRequest>,
//     ) -> Result<Response<ExportTraceServiceResponse>, Status> {
//         match self.handle(req.into_inner()).await {
//             Ok(res) => Ok(Response::new(res)),
//             Err(err) => Err(Status::internal(err.to_string())),
//         }
//     }
// }

// #[async_trait]
// impl LogsService for GrpcHandler {
//     async fn export(
//         &self,
//         req: Request<ExportLogsServiceRequest>,
//     ) -> Result<Response<ExportLogsServiceResponse>, Status> {
//         match self.handle(req.into_inner()).await {
//             Ok(res) => Ok(Response::new(res)),
//             Err(err) => Err(Status::internal(err.to_string())),
//         }
//     }
// }
