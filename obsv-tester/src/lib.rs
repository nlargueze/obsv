//! Testing framework for HTTP and gRPC

pub mod arg;
pub mod error;
pub mod grpc;
pub mod http;
pub mod test;
pub mod trace;

// /// Initializes ???
// fn init_tracing() {
//     // --> STDOUT
//     // let stdout_tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline()
//     //     .with_pretty_print(true)
//     //     .install_simple();

//     // // --> OTLP
//     // let otlp_endpoint = "http://localhost:4137".to_string();
//     // let otlp_srv_name = "testing-service".to_string();

//     // let otlp_exporter_cfg = opentelemetry_otlp::ExportConfig {
//     //     endpoint: otlp_endpoint,
//     //     timeout: Duration::from_secs(3),
//     //     protocol: opentelemetry_otlp::Protocol::Grpc,
//     // };

//     // let mut otlp_exporter_metadata = tonic::metadata::MetadataMap::new();
//     // otlp_exporter_metadata.insert("x-service-name", otlp_srv_name.parse().unwrap());

//     // let otlp_exporter = opentelemetry_otlp::new_exporter()
//     //     .tonic()
//     //     .with_export_config(otlp_exporter_cfg)
//     //     .with_metadata(otlp_exporter_metadata);
//     // // .with_tls_config(tonic::transport::ClientTlsConfig::new());
//     // let otlp_tracer = opentelemetry_otlp::new_pipeline()
//     //     .tracing()
//     //     .with_exporter(otlp_exporter)
//     //     .install_simple()
//     //     .unwrap();

//     // tracing subscriber
//     // let trc_layer = tracing_opentelemetry::layer().with_tracer(stdout_tracer);
//     // .with_tracer(otlp_tracer);

//     // let trc_subscriber = tracing_subscriber::Registry::default()
//     //     .with(tracing_subscriber::fmt::layer())
//     //     .with(tracing_subscriber::EnvFilter::from_default_env());
//     // tracing::subscriber::set_global_default(trc_subscriber).unwrap();
// }
