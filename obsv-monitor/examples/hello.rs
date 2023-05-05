//! Hello world example

use std::time::Duration;

use obsv_monitor::{export::stdout::StdoutExporter, monitor::http::HttpMonitor, MonitoringService};

#[tokio::main]
async fn main() {
    env_logger::init();

    let monitor_google = HttpMonitor::new("google", "https://www.google.com")
        .unwrap()
        .frequency(Duration::from_secs(1));
    let exporter_stdout = StdoutExporter::new("stdout");

    MonitoringService::new()
        .monitor(monitor_google)
        .exporter(exporter_stdout)
        .start_service()
        .await
        .unwrap();
}
