//! Processor
//!
//! The processor is responsible for processing received data

use async_trait::async_trait;

/// Processor
#[async_trait]
pub trait Processor: Send + Sync {
    /// Processes the data
    async fn process(&self);
}
