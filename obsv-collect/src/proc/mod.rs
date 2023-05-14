//! Processor
//!
//! The processor is responsible for processing received data

use async_trait::async_trait;
use obsv_core::Data;

pub mod id;

/// Processor
#[async_trait]
pub trait Processor: Send + Sync {
    /// Returns the processor ID
    fn id(&self) -> String;

    /// Processes the data
    async fn process(&self, data: Data) -> Data;
}
