//! Notification service

use obsv_notify::NotifService;

#[tokio::main]
async fn main() {
    let service = NotifService::new();
    eprintln!("starting notification service");
    service.start().await;
}
