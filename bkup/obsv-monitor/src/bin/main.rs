//! Monitoring service

use std::path::PathBuf;

use clap::Parser;
use obsv_monitor::cfg::MonitoringConfig;

/// All-in-one executable with all services
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the config TOML file (defaults to config.toml in the current directory)
    #[arg(short, long)]
    cfg: Option<PathBuf>,
}

/// Default config file
const DEFAULT_CFG_FILE: &str = "monitor.cfg.toml";

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // load the config
    let cfg_path = cli.cfg.unwrap_or_else(|| PathBuf::from(DEFAULT_CFG_FILE));
    let cfg = MonitoringConfig::from_file(&cfg_path).unwrap();

    // start the service
    let service = cfg.service().unwrap();
    eprintln!("Starting monitoring service ...");
    service.start().await.unwrap();
}
