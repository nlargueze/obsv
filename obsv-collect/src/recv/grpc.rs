//! OpenTelemetry gRPC receiver

use std::net::SocketAddr;

use async_trait::async_trait;
use obsv_core::{
    conn::otlp::proto::collector::trace::v1::{
        trace_service_server, ExportTracePartialSuccess, ExportTraceServiceRequest,
        ExportTraceServiceResponse,
    },
    Data,
};
use tokio::sync::mpsc::UnboundedSender;
use tonic::{Request, Response};

use super::Receiver;

/// GRPC receiver
///
/// This receiver implements the OpenTelemetry GRPC receiver specs.
pub struct GrpcReceiver {
    /// Address
    addr: SocketAddr,
}

impl GrpcReceiver {
    /// Instantiates a new GRPC receiver
    pub fn new(addr: &str) -> Self {
        let addr = addr.parse().unwrap();
        Self { addr }
    }
}

#[async_trait]
impl Receiver for GrpcReceiver {
    fn id(&self) -> String {
        "receiver_grpc".to_string()
    }

    async fn start(&self, tx: UnboundedSender<Data>) {
        let service = trace_service_server::TraceServiceServer::new(TraceHandler::new(
            self.id().as_str(),
            tx,
        ));
        tonic::transport::Server::builder()
            .add_service(service)
            .serve(self.addr)
            .await
            .unwrap();
    }
}

/// GRPC service
#[derive(Clone)]
struct TraceHandler {
    /// ID
    id: String,
    /// Channel sender
    tx: UnboundedSender<Data>,
}

impl TraceHandler {
    pub fn new(id: &str, tx: UnboundedSender<Data>) -> Self {
        Self {
            id: id.to_string(),
            tx,
        }
    }
}

#[tonic::async_trait]
impl trace_service_server::TraceService for TraceHandler {
    async fn export(
        &self,
        req: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, tonic::Status> {
        log::trace!("[{}] Received data", self.id);

        // sending to channel
        let (_, _, req) = req.into_parts();
        let data: Data = Data::Spans(req.into());
        match self.tx.send(data) {
            Ok(_ok) => Ok(Response::new(ExportTraceServiceResponse {
                partial_success: None,
            })),
            Err(err) => {
                log::error!("Error sending data to channel");
                return Ok(Response::new(ExportTraceServiceResponse {
                    partial_success: Some(ExportTracePartialSuccess {
                        rejected_spans: 0,
                        error_message: err.to_string(),
                    }),
                }));
            }
        }
    }
}
