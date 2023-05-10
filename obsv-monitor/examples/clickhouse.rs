//! Simple example with clickhouse

use std::time::Duration;

use obsv_monitor::{
    export::{clickhouse::ClickhouseExporter, stdout::StdoutExporter},
    monitor::http::HttpMonitor,
    MonitoringService,
};

#[tokio::main]
async fn main() {
    real_main().await;
}

async fn real_main() {
    env_logger::init();

    let monitor_google = HttpMonitor::new("google", "http://www.google.com")
        .unwrap()
        .frequency(Duration::from_secs(5));
    let exporter_stdout = StdoutExporter::new("stdout");
    let exporter_ch = ClickhouseExporter::new("clickhouse", "http://localhost:8123", "test");

    MonitoringService::new()
        .monitor(monitor_google)
        .exporter(exporter_stdout)
        .exporter(exporter_ch)
        .start_service()
        .await
        .unwrap();
}
