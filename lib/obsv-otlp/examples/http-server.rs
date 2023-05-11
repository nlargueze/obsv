//! HTTP server

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use obsv_otlp::proto::collector::trace::v1::ExportTraceServiceRequest;
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:4318".parse::<SocketAddr>().unwrap();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handler)) });

    eprintln!("Listening on http://{addr} ...");
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

/// Handler for HTTP requests
async fn handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "") => Ok(Response::new("Hello, World".into())),
        (&Method::GET, "/up") => Ok(Response::new("UP".into())),
        (&Method::POST, "/v1/traces") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let service_req: ExportTraceServiceRequest = match serde_json::from_slice(&body_bytes) {
                Ok(ok) => ok,
                Err(err) => {
                    let res = Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from(err.to_string()))
                        .unwrap();
                    return Ok(res);
                }
            };
            println!("{service_req:#?}");
            Ok(Response::new("OK".into()))
        }
        _ => {
            let res = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(format!("Not Found {} {}", req.method(), req.uri().path()).into())
                .unwrap();
            Ok(res)
        }
    }
}
