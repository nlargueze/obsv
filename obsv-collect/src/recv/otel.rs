//! OpenTelemetry receiver

use opentelemetry_sdk::trace::Span;

/// OpenTelemetry receiver
pub struct OtelReceiver {}

impl OtelReceiver {
    /// Start the HTTP OTLP receiver
    pub async fn start_http(&self) {}

    /// Start the GRPC OTLP receiver
    pub async fn start_grpc(&self) {}
}
