//! This crate provides a simple HTTP server for the API

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

/// API server
#[derive(Debug)]
pub struct ApiServer {
    addr: SocketAddr,
}

impl ApiServer {
    /// Creates a new Server
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Starts the server
    pub async fn start(self) {
        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_req)) });
        let server = hyper::Server::bind(&self.addr).serve(make_svc);
        server.await.unwrap();
    }
}

/// Handler
async fn handle_req(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap()),
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    }
}
