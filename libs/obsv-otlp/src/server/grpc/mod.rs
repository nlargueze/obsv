//! GRPC server

#[cfg(test)]
mod tests;

use std::{
    future::{self, Future},
    net::SocketAddr,
};

use tonic::{Request, Response, Status};

use crate::proto::collector::{
    logs::v1::{
        logs_service_server::{LogsService, LogsServiceServer},
        ExportLogsServiceRequest, ExportLogsServiceResponse,
    },
    metrics::v1::{
        metrics_service_server::{MetricsService, MetricsServiceServer},
        ExportMetricsServiceRequest, ExportMetricsServiceResponse,
    },
    trace::v1::{
        trace_service_server::{TraceService, TraceServiceServer},
        ExportTraceServiceRequest, ExportTraceServiceResponse,
    },
};

/// OTLP GROC server
pub struct GrpcServer<T, U, V, F>
where
    T: TraceService,
    U: LogsService,
    V: MetricsService,
    F: Future<Output = ()>,
{
    /// Address
    pub addr: SocketAddr,
    /// Shutown signal
    pub shutdown: F,
    /// Trace service
    pub trace_service: T,
    /// Logs service
    pub logs_service: U,
    /// Metrics service
    pub metrics_service: V,
}

impl GrpcServer<NoopTraceService, NoopLogsService, NoopMetricsService, future::Pending<()>> {
    /// Creates a new GRPC server
    pub fn new(
    ) -> GrpcServer<NoopTraceService, NoopLogsService, NoopMetricsService, future::Pending<()>>
    {
        GrpcServer {
            addr: "127.0.0.1:4317".parse().unwrap(),
            shutdown: future::pending::<()>(),
            trace_service: NoopTraceService,
            logs_service: NoopLogsService,
            metrics_service: NoopMetricsService,
        }
    }
}

impl Default
    for GrpcServer<NoopTraceService, NoopLogsService, NoopMetricsService, future::Pending<()>>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U, V, F> GrpcServer<T, U, V, F>
where
    T: TraceService,
    U: LogsService,
    V: MetricsService,
    F: Future<Output = ()>,
{
    /// Sets the address
    pub fn addr(mut self, addr: &str) -> Self {
        self.addr = addr.parse().expect("Invalid address");
        self
    }

    /// Sets the shutdown signal
    pub fn shutdown<S>(self, f: S) -> GrpcServer<T, U, V, S>
    where
        S: Future<Output = ()>,
    {
        GrpcServer {
            addr: self.addr,
            shutdown: f,
            trace_service: self.trace_service,
            logs_service: self.logs_service,
            metrics_service: self.metrics_service,
        }
    }

    /// Sets the trace service
    pub fn trace_service<S>(self, service: S) -> GrpcServer<S, U, V, F>
    where
        S: TraceService,
    {
        GrpcServer {
            addr: self.addr,
            shutdown: self.shutdown,
            trace_service: service,
            logs_service: self.logs_service,
            metrics_service: self.metrics_service,
        }
    }

    /// Sets the logs service
    pub fn logs_service<S>(self, service: S) -> GrpcServer<T, S, V, F>
    where
        S: LogsService,
    {
        GrpcServer {
            addr: self.addr,
            shutdown: self.shutdown,
            trace_service: self.trace_service,
            logs_service: service,
            metrics_service: self.metrics_service,
        }
    }

    /// Sets the metrics service
    pub fn metrics_service<S>(self, service: S) -> GrpcServer<T, U, S, F>
    where
        S: MetricsService,
    {
        GrpcServer {
            addr: self.addr,
            shutdown: self.shutdown,
            trace_service: self.trace_service,
            logs_service: self.logs_service,
            metrics_service: service,
        }
    }

    /// Starts the service
    pub async fn start(self) -> Result<(), tonic::transport::Error> {
        tonic::transport::Server::builder()
            .add_service(TraceServiceServer::new(self.trace_service))
            .add_service(LogsServiceServer::new(self.logs_service))
            .add_service(MetricsServiceServer::new(self.metrics_service))
            .serve_with_shutdown(self.addr, self.shutdown)
            .await
    }
}

/// A trace service that does nothing
pub struct NoopTraceService;

#[tonic::async_trait]
impl TraceService for NoopTraceService {
    async fn export(
        &self,
        _request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}

/// A logs service that does nothing
pub struct NoopLogsService;

#[tonic::async_trait]
impl LogsService for NoopLogsService {
    async fn export(
        &self,
        _request: Request<ExportLogsServiceRequest>,
    ) -> Result<Response<ExportLogsServiceResponse>, Status> {
        Ok(Response::new(ExportLogsServiceResponse {
            partial_success: None,
        }))
    }
}

/// A metrics service that does nothing
pub struct NoopMetricsService;

#[tonic::async_trait]
impl MetricsService for NoopMetricsService {
    async fn export(
        &self,
        _request: Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, Status> {
        Ok(Response::new(ExportMetricsServiceResponse {
            partial_success: None,
        }))
    }
}
