//! A simple test service to send telemetry data

use std::time::Duration;

use opentelemetry_otlp::WithExportConfig;
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

    // init the tracer
    init_tracer();

    // Generate tests
    gen_traces();
}

/// Initializes the tracer
fn init_tracer() {
    // STDOUT
    let stdout_tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline()
        .with_pretty_print(true)
        .install_simple();

    // OTLP
    let otlp_exporter_cfg = opentelemetry_otlp::ExportConfig {
        endpoint: "http://0.0.0.0:4317".to_string(),
        timeout: Duration::from_secs(3),
        protocol: opentelemetry_otlp::Protocol::Grpc,
    };

    let mut otlp_exporter_metadata = tonic::metadata::MetadataMap::new();
    otlp_exporter_metadata.insert("x-metadata-1", "test metadata".parse().unwrap());

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_export_config(otlp_exporter_cfg)
        .with_metadata(otlp_exporter_metadata);
    // .with_tls_config(tonic::transport::ClientTlsConfig::new());
    let otlp_tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_simple()
        .unwrap();

    // tracing subscriber
    let trc_layer = tracing_opentelemetry::layer()
        .with_tracer(stdout_tracer)
        .with_tracer(otlp_tracer);

    let trc_subscriber = tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(trc_layer);
    tracing::subscriber::set_global_default(trc_subscriber).unwrap();
}

/// Sends an OTEL trace
#[tracing::instrument]
fn gen_traces() {
    tracing::info!("Generating traces");

    test_levels()
}

/// Send traces with different levels
#[tracing::instrument]
fn test_levels() {
    tracing::trace!("**trace**");
    tracing::debug!("**debug**");
    tracing::info!("**info**");
    tracing::warn!("**warn**");
    tracing::error!("**error**");
}
