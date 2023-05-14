//! Receiver

use async_trait::async_trait;

#[cfg(feature = "http")]
pub mod http;

/// Receiver
#[async_trait]
pub trait Receiver: Send + Sync {
    /// Starts receiving metrics/traces/logs/etc data
    async fn start(&self);
}
