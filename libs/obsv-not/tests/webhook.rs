//! Webhook tests

use std::future::Future;

use obsv_not::{
    error::Error,
    webhook::{WebhookChannel, WebhookMessage},
    GenericChannel,
};
use salvo::prelude::*;
use serde::Serialize;
use tokio::sync::oneshot::{self, Sender};

#[tokio::test]
async fn test_webhook() {
    // shutdown signal
    let (tx, rx) = oneshot::channel::<()>();

    let task_server = tokio::spawn(async {
        let signal = async {
            rx.await.unwrap();
        };
        start_server(signal).await;
    });

    let task_test = tokio::spawn(async {
        send_webhook(tx).await;
    });

    tokio::try_join!(task_server, task_test).unwrap();
}

/// Starts the server
async fn start_server<T>(signal: T)
where
    T: Future<Output = ()> + Send + 'static,
{
    let router = Router::new().get(get_root).post(recv_webhook);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor)
        .serve_with_graceful_shutdown(router, signal, None)
        .await;
}

/// Root handler
#[handler]
async fn get_root() -> &'static str {
    "OK"
}

/// Receives the webhook
#[handler]
async fn recv_webhook(req: &mut Request) {
    eprintln!("Webhook received: {:?}", req);
}

/// Message
#[derive(Debug, Serialize)]
struct Message {
    message: String,
}

impl TryFrom<Message> for WebhookMessage {
    type Error = Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        WebhookMessage::new().body(value)
    }
}

/// Sends a webhook
pub async fn send_webhook(tx: Sender<()>) {
    let channel = WebhookChannel::builder("http://localhost:5800")
        .name("test")
        .build()
        .unwrap();

    channel.get_origin().await.unwrap();

    let message = Message {
        message: "Hello".to_string(),
    };
    channel.send(message).await.unwrap();

    tx.send(()).unwrap();
}
