//! Processor
//!
//! The processor is responsible for processing received data

use async_trait::async_trait;
use obsv_core::Data;

/// Processor
#[async_trait]
pub trait Processor: Send + Sync {
    /// Processes the data
    async fn process(&self, data: Data) -> Data;
}

/// Identity processor
#[derive(Debug, Clone)]
pub struct PassThroughProcessor {}

#[async_trait]
impl Processor for PassThroughProcessor {
    async fn process(&self, data: Data) -> Data {
        data
    }
}
