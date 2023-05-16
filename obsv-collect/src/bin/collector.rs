//! Default collector

use obsv_collect::{
    expt::{file::FileExporter, stdout::StdoutExporter},
    recv::{grpc::GrpcReceiver, http::HttpReceiver},
    CollService,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let http_receiver = HttpReceiver::new("0.0.0.0:4318");
    let grpc_receiver = GrpcReceiver::new("0.0.0.0:4317");
    let stdout_exporter = StdoutExporter::new();
    let log_file = std::env::temp_dir().join("logs.txt");
    let file_exporter = FileExporter::new(&log_file);

    eprintln!("[collector] receiver: HTTP listening on 0.0.0.0:4318");
    eprintln!("[collector] receiver: GRPC listening on 0.0.0.0:4317");
    eprintln!("[collector] exporter: stdout");
    eprintln!("[collector] exporter: file ({})", log_file.display());

    CollService::new()
        .receiver(http_receiver)
        .receiver(grpc_receiver)
        .exporter(stdout_exporter)
        .exporter(file_exporter)
        .start()
        .await;
}
