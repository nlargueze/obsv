//! CLI tool to perform HTTP/gRPC request testing

use std::process::exit;

use clap::Parser;
use colored::Colorize;
use obsv_tester::{arg, http};

#[tokio::main]
async fn main() {
    // parse the arguments
    let cli = arg::Cli::parse();
    // println!("{cli:#?}");

    let tests = match cli.commands {
        arg::Commands::Http(args) => match http::send_requests(args).await {
            Ok(coll) => coll,
            Err(err) => {
                eprintln!("{}", "ERROR:".red());
                eprintln!("{}", format!("{err}").red());
                exit(1);
            }
        },
        arg::Commands::Grpc { url: _ } => todo!(),
    };

    // print the results
    println!();
    tests.print();
}

// /// Initializes the tracer
// fn init_tracer() {
//     // read the config
//     let endpoint = "http://localhost:4137".to_string();
//     let srv_name = "testing-service".to_string();

//     // STDOUT
//     let stdout_tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline()
//         .with_pretty_print(true)
//         .install_simple();

//     // OTLP
//     let otlp_exporter_cfg = opentelemetry_otlp::ExportConfig {
//         endpoint,
//         timeout: Duration::from_secs(3),
//         protocol: opentelemetry_otlp::Protocol::Grpc,
//     };

//     let mut otlp_exporter_metadata = tonic::metadata::MetadataMap::new();
//     otlp_exporter_metadata.insert("x-service-name", srv_name.parse().unwrap());

//     let otlp_exporter = opentelemetry_otlp::new_exporter()
//         .tonic()
//         .with_export_config(otlp_exporter_cfg)
//         .with_metadata(otlp_exporter_metadata);
//     // .with_tls_config(tonic::transport::ClientTlsConfig::new());
//     let otlp_tracer = opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(otlp_exporter)
//         .install_simple()
//         .unwrap();

//     // tracing subscriber
//     let trc_layer = tracing_opentelemetry::layer()
//         .with_tracer(stdout_tracer)
//         .with_tracer(otlp_tracer);

//     let trc_subscriber = tracing_subscriber::Registry::default()
//         .with(tracing_subscriber::fmt::layer())
//         .with(tracing_subscriber::EnvFilter::from_default_env())
//         .with(trc_layer);
//     tracing::subscriber::set_global_default(trc_subscriber).unwrap();
// }
