//! Notification channels

use async_trait::async_trait;

use crate::error::Error;

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "webhook")]
pub mod webhook;

/// A messaging channel
#[async_trait]
pub trait Channel: Send + Sync {
    /// Sends a message
    async fn send(&self, message: &str) -> Result<(), Error>;
}
