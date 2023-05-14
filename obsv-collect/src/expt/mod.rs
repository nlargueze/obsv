//! Exporter
//!
//! The exporter is responsible for exporting the received data

use async_trait::async_trait;

/// Exporter
#[async_trait]
pub trait Exporter: Send + Sync {
    /// Exports data
    async fn export(&self);
}
