//! CLI tool to perform HTTP/gRPC request testing

use std::process::exit;

use clap::Parser;
use colored::Colorize;
use obsv_tester::CliArgs;

#[tokio::main]
async fn main() {
    let cli = CliArgs::parse();
    // println!("{cli:#?}");

    let tests = match obsv_tester::exec(cli).await {
        Ok(tests) => tests,
        Err(err) => {
            eprintln!("{}", "ERROR:".red());
            eprintln!("{}", format!("{err}").red());
            exit(1);
        }
    };

    let stats = tests.stats();
    println!();
    println!("{stats}")
}
