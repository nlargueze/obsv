//! A simple test service to send telemetry data

use serde::Deserialize;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

/// Configuration
#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct AppConfig {}

#[tokio::main]
async fn main() {
    // read the config
    let _cfg: AppConfig = config::Config::builder()
        .add_source(
            config::Environment::with_prefix("TESTER")
                .try_parsing(true)
                .separator("_")
                .list_separator(" "),
        )
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    // setup telemetry
    let otel_tracer_stdout = opentelemetry::sdk::export::trace::stdout::new_pipeline()
        .with_pretty_print(true)
        .install_simple();
    let otel_exporter_otlp = opentelemetry_otlp::new_exporter().tonic();
    let otel_tracer_otlp = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otel_exporter_otlp)
        .install_simple()
        .unwrap();

    let otel_layer = tracing_opentelemetry::layer()
        .with_tracer(otel_tracer_stdout)
        .with_tracer(otel_tracer_otlp);
    let trc_subscriber = tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(otel_layer);
    tracing::subscriber::set_global_default(trc_subscriber).unwrap();

    // Send tests
    send_trace();
    println!("Test traces sent!")
}

/// Sends an OTEL trace
#[tracing::instrument]
fn send_trace() {
    tracing::info!("Dummy trace");
}
