//! All-in-one executable

use std::path::PathBuf;

use clap::Parser;
use obsv_bin::cfg::Config;

/// All-in-one executable with all services
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the config TOML file (defaults to config.toml in the current directory)
    #[arg(short, long)]
    cfg: Option<PathBuf>,
}

// Config file name
const CFG_FILE_NAME: &str = "obsv.cfg.toml";

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // load the config
    let cfg_path = cli.cfg.unwrap_or_else(|| PathBuf::from(CFG_FILE_NAME));
    let cfg = Config::from_file(&cfg_path).unwrap();

    // Collect all the tasks
    let mut tasks = Vec::new();

    // Setup the monitoring service
    tasks.push(tokio::spawn(async move {
        let service = cfg.monitoring.service().unwrap();
        service.start().await
    }));

    // Join all the tasks
    for task in tasks {
        task.await.expect("Panic in task").unwrap();
    }
}
