//! OpenTelemetry HTTP receiver

use std::{convert::Infallible, net::SocketAddr};

use async_trait::async_trait;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use obsv_otlp::proto::collector::trace::v1::ExportTraceServiceRequest;

use super::Receiver;

/// HTTP receiver
///
/// This receiver implements the OpenTelemetry HTTP receiver specs.
pub struct HttpReceiver {
    /// Address
    addr: SocketAddr,
}

#[async_trait]
impl Receiver for HttpReceiver {
    /// Start the HTTP OTLP receiver
    async fn start(&self) {
        //     let make_svc = make_service_fn(|_conn| async {
        //         Ok::<_, Infallible>(service_fn(|req| {
        //             async {
        //                 Response::new("Hello, World".to_string())

        //                 // match (req.method(), req.uri().path()) {
        //                 //     (&Method::GET, "") => Ok(Response::new("Hello, World".into())),
        //                 //     (&Method::GET, "/up") => Ok(Response::new("UP".into())),
        //                 //     (&Method::POST, "/v1/traces") => {
        //                 //         // parse the trace
        //                 //         let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
        //                 //         let service_req: ExportTraceServiceRequest =
        //                 //             match serde_json::from_slice(&body_bytes) {
        //                 //                 Ok(ok) => ok,
        //                 //                 Err(err) => {
        //                 //                     todo!();
        //                 //                     // return Ok(Response::builder()
        //                 //                     //     .status(StatusCode::BAD_REQUEST)
        //                 //                     //     .body(Body::from(err.to_string()))
        //                 //                     //     .unwrap());
        //                 //                 }
        //                 //             };
        //                 //         // println!("{service_req:#?}");
        //                 //         Ok(Response::new("OK".into()))
        //                 //     }
        //                 //     _ => Ok(Response::builder()
        //                 //         .status(StatusCode::NOT_FOUND)
        //                 //         .body(format!("Not Found {} {}", req.method(), req.uri().path()).into())
        //                 //         .unwrap()),
        //                 // }
        //             }
        //         }))
        //     });

        let make_svc = make_service_fn(|_conn| async {
            // s
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(move |_req: Request<Body>| async {
                Ok(Response::new("Hello, World".into()))
            }))
        });

        let addr = self.addr;
        eprintln!("Listening on http://{addr} ...");
        let server = Server::bind(&addr).serve(make_svc);
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}
