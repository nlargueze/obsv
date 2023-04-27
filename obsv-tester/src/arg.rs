//! CLI arguments

use clap::{Parser, Subcommand};

use crate::http::HttpRequestArgs;

/// CLI to send HTTP and gRPC requests
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = Some("ssss"))]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

/// CLI commands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Send a HTTP request
    Http(HttpRequestArgs),
    /// Send a GRPC request
    Grpc {
        /// URL (eg localhost:8080, :8080, ...)
        /// - :1234
        url: String,
    },
}
