//! OpenTelemetry HTTP receiver

use std::{convert::Infallible, net::SocketAddr};

use async_trait::async_trait;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use obsv_core::{conn::otlp::ExportTraceServiceRequest, Data};
use tokio::sync::mpsc::UnboundedSender;

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
    /// Start the HTTP OTLP receiver
    async fn start(&self, tx: UnboundedSender<Data>) {
        let make_svc = make_service_fn(|conn: &AddrStream| {
            let addr = conn.remote_addr();
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
    match (req.method(), req.uri().path()) {
        (&Method::GET, "") => Ok(Response::new("Hello, World".into())),
        (&Method::GET, "/up") => Ok(Response::new("UP".into())),
        (&Method::POST, "/v1/traces") => {
            log::trace!("Received HTTP POST /v1/traces");
            // parse the trace
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
            if let Err(err) = tx.send(otlp_req.into()) {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(err.to_string()))
                    .unwrap());
            } else {
                println!("Data sent from HTTP to processor");
            }

            // ok
            Ok(Response::new(Body::empty()))
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("Not Found {} {}", req.method(), req.uri().path()).into())
            .unwrap()),
    }
}
