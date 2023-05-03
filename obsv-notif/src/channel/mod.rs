//! Notification channels

use async_trait::async_trait;

use crate::{error::Error, Notification};

pub mod builtin;

/// A notification channel
#[async_trait]
pub trait Channel {
    /// Sends a notification.
    async fn send(&self, notif: impl Notification + Send) -> Result<(), Error>;
}
