//! Testing framework for HTTP and gRPC

use clap::{Parser, Subcommand};
use error::Error;
use test::TestSuite;

pub mod error;
pub mod grpc;
pub mod http;
pub mod test;
pub mod trace;

/// CLI to send HTTP and gRPC requests
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub commands: RequestType,
}

/// Request type
#[derive(Debug, Subcommand)]
pub enum RequestType {
    /// Send a HTTP request
    Http(http::HttpRequestArgs),
    /// Send a GRPC request
    Grpc {
        /// URL (eg localhost:8080, :8080, ...)
        /// - :1234
        url: String,
    },
}

/// Executes the test suite
pub async fn exec(args: CliArgs) -> Result<TestSuite, Error> {
    match args.commands {
        RequestType::Http(args) => http::send_requests(args).await,
        RequestType::Grpc { url: _ } => todo!(),
    }
}
