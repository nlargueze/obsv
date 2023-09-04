//! OpenTelemetry HTTP receiver

use std::{convert::Infallible, net::SocketAddr};

use async_trait::async_trait;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use obsv_core::conn::otlp::proto::collector::{
    logs::v1::ExportLogsServiceRequest, metrics::v1::ExportMetricsServiceRequest,
    trace::v1::ExportTraceServiceRequest,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::Data;

use super::Receiver;

/// HTTP receiver
///
/// This receiver implements the OpenTelemetry HTTP receiver specs.
pub struct HttpReceiver {
    /// Address
    addr: SocketAddr,
}

impl HttpReceiver {
    /// Instantiates a new HTTP receiver
    pub fn new(addr: &str) -> Self {
        let addr = addr.parse().unwrap();
        Self { addr }
    }
}

#[async_trait]
impl Receiver for HttpReceiver {
    async fn start(&self, tx: UnboundedSender<Data>) {
        let make_svc = make_service_fn(|conn: &AddrStream| {
            let _addr = conn.remote_addr();
            let tx = tx.clone();
            async {
                // service_fn converts our function into a `Service`
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    let tx = tx.clone();
                    async move { handle_req(tx, req).await }
                }))
            }
        });

        let addr = self.addr;
        let server = Server::bind(&addr).serve(make_svc);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

/// Handles the request
pub async fn handle_req(
    tx: UnboundedSender<Data>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    log::trace!("received HTTP request");
    match (req.method(), req.uri().path()) {
        (&Method::GET, "") => Ok(Response::new("Hello, World".into())),
        (&Method::GET, "/up") => Ok(Response::new("UP".into())),
        // OTLP/HTTP trace collector
        (&Method::POST, "/v1/traces") => {
            // parse the OTLP trace as JSON payload
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let otlp_req: ExportTraceServiceRequest = match serde_json::from_slice(&body_bytes) {
                Ok(ok) => ok,
                Err(err) => {
                    log::error!("Invalid HTTP request body: {}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(err.to_string()))
                        .unwrap());
                }
            };
            // sending to channel
            todo!("handle http request for traces");
            // if let Err(err) = tx.send(otlp_req.into()) {
            //     log::error!("Error sending data to channel: {err}");
            // }
            // // ok
            // Ok(Response::new(Body::empty()))
        }
        // OTLP/HTTP metrics collector
        (&Method::POST, "/v1/metrics") => {
            // parse the OTLP trace as JSON payload
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let otlp_req: ExportMetricsServiceRequest = match serde_json::from_slice(&body_bytes) {
                Ok(ok) => ok,
                Err(err) => {
                    log::error!("Invalid HTTP request body: {}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(err.to_string()))
                        .unwrap());
                }
            };
            // sending to channel
            todo!("handle http request for metrics");
            // if let Err(err) = tx.send(otlp_req.into()) {
            //     log::error!("Error sending data to channel: {err}");
            // }
            // // ok
            // Ok(Response::new(Body::empty()))
        }
        // OTLP/HTTP logs collector
        (&Method::POST, "/v1/logs") => {
            // parse the OTLP trace as JSON payload
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let otlp_req: ExportLogsServiceRequest = match serde_json::from_slice(&body_bytes) {
                Ok(ok) => ok,
                Err(err) => {
                    log::error!("Invalid HTTP request body: {}", err);
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(err.to_string()))
                        .unwrap());
                }
            };
            // sending to channel
            todo!("handle http request for logs");
            // if let Err(err) = tx.send(otlp_req.into()) {
            //     log::error!("Error sending data to channel: {err}");
            // }
            // // ok
            // Ok(Response::new(Body::empty()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("Not Found {} {}", req.method(), req.uri().path()).into())
            .unwrap()),
    }
}
