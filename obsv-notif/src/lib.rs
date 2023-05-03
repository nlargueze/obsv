//! This crate provides the utilities for notifications.
//!
//! It is structured around the [Channel] trait, which represents a notification channel.
//! A number of built-in channels are provided.
//!
//! # Features
//!
//! - **webhook**: webhook channel
//! - **email**: email channel

pub mod channel;
pub mod error;

use serde::Serialize;

pub use async_trait::async_trait;

/// A trait that represents a notification
pub trait Notification: Serialize {
    /// Returns the main message as a string
    fn message(&self) -> String;
}

/// A simple text notificaton
#[derive(Debug, Clone, Serialize)]
pub struct TextNotification {
    /// Message
    pub message: String,
}

impl TextNotification {
    /// Instantiate a new notification
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Notification for TextNotification {
    fn message(&self) -> String {
        self.message.clone()
    }
}
