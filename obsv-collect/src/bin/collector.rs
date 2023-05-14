//! Default collector

use obsv_collect::{recv::http::HttpReceiver, Server};

#[tokio::main]
async fn main() {
    env_logger::init();

    let http_receiver = HttpReceiver::new("0.0.0.0:4318");
    eprintln!("HTTP listening on :4318 ...");

    Server::new().receiver(http_receiver).start().await;
}
