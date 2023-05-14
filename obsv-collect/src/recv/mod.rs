//! Receiver

use async_trait::async_trait;
use obsv_core::Data;
use tokio::sync::mpsc::UnboundedSender;

#[cfg(feature = "grpc")]
pub mod grpc;
#[cfg(feature = "http")]
pub mod http;

/// Receiver
#[async_trait]
pub trait Receiver: Send + Sync {
    /// Starts receiving metrics/traces/logs/etc data
    async fn start(&self, tx: UnboundedSender<Data>);
}
