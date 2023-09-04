//! HTTP server

use prost::Message;
use serde::{de::DeserializeOwned, Serialize};

use crate::proto::collector::{
    logs::v1::{ExportLogsServiceRequest, ExportLogsServiceResponse},
    metrics::v1::{ExportMetricsServiceRequest, ExportMetricsServiceResponse},
    trace::v1::{ExportTraceServiceRequest, ExportTraceServiceResponse},
};

#[cfg(test)]
mod tests;

/// The HTTP endpoint for traces
pub const ENDPOINT_TRACES: &str = "/v1/traces";

/// The HTTP endpoint for logs
pub const ENDPOINT_LOGS: &str = "/v1/logs";

/// The HTTP endpoint for metrics
pub const ENDPOINT_METRICS: &str = "/v1/metrics";

/// MIME type for JSON
pub const APPLICATION_JSON: &str = "application/json";

/// MIME type for PROTOBUF
pub const APPLICATION_PROTOBUF: &str = "application/x-protobuf";

/// A trait to map OTLP objects to HTTP requests and responses
pub trait HttpConvert: Sized + Message + Serialize + Default + DeserializeOwned {
    /// Serializes an OTLP object
    fn into_http_body(self, content_type: &str) -> Result<Vec<u8>, String> {
        match content_type {
            APPLICATION_PROTOBUF => {
                let mut buf = vec![];
                self.encode(&mut buf)
                    .map_err(|err| format!("Cannot encode protobuf: {}", err))?;
                Ok(buf)
            }
            APPLICATION_JSON => {
                serde_json::to_vec(&self).map_err(|err| format!("Cannot encode JSON: {}", err))
            }
            _ => Err(format!("Invalid content-type: {content_type}")),
        }
    }

    /// Deserializes an OTLP object
    fn from_http_request(content_type: &str, body: &[u8]) -> Result<Self, String> {
        match content_type {
            APPLICATION_PROTOBUF => {
                Self::decode(body).map_err(|err| format!("Cannot decode protobuf: {}", err))
            }
            APPLICATION_JSON => {
                serde_json::from_slice(body).map_err(|err| format!("Cannot decode JSON: {}", err))
            }
            _ => Err(format!("Invalid content-type: {content_type}")),
        }
    }
}

impl HttpConvert for ExportTraceServiceRequest {}
impl HttpConvert for ExportTraceServiceResponse {}
impl HttpConvert for ExportLogsServiceRequest {}
impl HttpConvert for ExportLogsServiceResponse {}
impl HttpConvert for ExportMetricsServiceRequest {}
impl HttpConvert for ExportMetricsServiceResponse {}
