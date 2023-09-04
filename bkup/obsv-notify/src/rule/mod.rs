//! Rule
//!
//! This module provides a [Rule] trait which defines how we should check for

use async_trait::async_trait;

use crate::error::Error;

pub mod db;

/// A rule checks if a notification should be sent
#[async_trait]
pub trait Rule: Send + Sync {
    /// Checks if an event has happend which requires notification
    ///
    /// It returns `None` if no notification should be sent, or an option with a message
    async fn check(&self) -> Result<Option<String>, Error>;
}
