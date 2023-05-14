//! Exporter
//!
//! The exporter is responsible for exporting the received data

use async_trait::async_trait;
use obsv_core::Data;

pub mod file;
pub mod stdout;

/// Exporter
#[async_trait]
pub trait Exporter: Send + Sync {
    /// Returns the processor ID
    fn id(&self) -> String;

    /// Exports data
    async fn export(&self, data: Data);
}
