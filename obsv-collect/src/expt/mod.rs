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
    /// Exports data
    async fn export(&self, data: Vec<Data>);
}
