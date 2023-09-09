//! This crate provides the utilities for notifications.
//!
//! It is structured around the [Channel] trait, which represents a notification channel.
//! A number of built-in channels are provided.
//!
//! # Features
//!
//! - **webhook**: webhook channel
//! - **email**: email channel

#[cfg(feature = "email")]
pub mod email;
pub mod error;
#[cfg(feature = "webhook")]
pub mod webhook;

pub use async_trait::async_trait;
use error::Error;

/// A notification channel
#[async_trait]
pub trait Channel: Send + Sync {
    /// A channel message
    type Message: Send;

    /// Sends a message
    async fn send(&self, message: Self::Message) -> Result<(), Error>;
}

/// A generic notification channel
#[async_trait]
pub trait GenericChannel<T> {
    async fn send(&self, message: T) -> Result<(), Error>;
}

// NB: implement GenericChannel for all Channel impls
#[async_trait]
impl<T, M> GenericChannel<M> for T
where
    T: Channel,
    M: TryInto<T::Message, Error = Error> + Send + 'static,
{
    async fn send(&self, message: M) -> Result<(), Error> {
        self.send(message.try_into()?).await
    }
}
