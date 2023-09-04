//! HTTP server

// use async_trait::async_trait;
// use obsv_otlp::proto::collector::logs::v1::{ExportLogsServiceRequest, ExportLogsServiceResponse};
// use obsv_otlp::proto::collector::trace::v1::{
//     ExportTraceServiceRequest, ExportTraceServiceResponse,
// };
// use salvo::http::ResBody;
// use salvo::{hyper::header::CONTENT_TYPE, prelude::*};

#[tokio::main]
async fn main() {}

// /// HTTP trace handler
// #[derive(Debug)]
// struct HttpTraceHandler;

// #[async_trait]
// impl OtlpHandler<ExportTraceServiceRequest, ExportTraceServiceResponse> for HttpTraceHandler {
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

// /// HTTP logs handler
// #[derive(Debug)]
// struct HttpLogsHandler;

// #[async_trait]
// impl OtlpHandler<ExportLogsServiceRequest, ExportLogsServiceResponse> for HttpLogsHandler {
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

// /// Handler for logs
// #[handler]
// async fn handle_logs(req: &mut Request, res: &mut Response) {
//     let handler = HttpLogsHandler;

//     let content_type = req.header::<String>(CONTENT_TYPE).unwrap_or_default();
//     let body = match req.parse_body::<Vec<u8>>().await {
//         Ok(b) => b,
//         Err(err) => {
//             res.status_code(StatusCode::BAD_REQUEST)
//                 .render(format!("Cannot parse requets body: {err}"));
//             return;
//         }
//     };
//     match handler.handle_http(&content_type, &body).await {
//         Ok(body) => {
//             res.status_code(StatusCode::OK)
//                 .add_header(CONTENT_TYPE, content_type, true)
//                 .unwrap()
//                 .body(ResBody::Once(body.into()));
//         }
//         Err(err) => {
//             res.status_code(StatusCode::BAD_REQUEST)
//                 .render(format!("Cannot process request: {err}"));
//         }
//     }
// }
