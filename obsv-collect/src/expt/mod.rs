//! Exporter
//!
//! The exporter is responsible for exporting the received data

use async_trait::async_trait;
use dyn_clone::DynClone;

use crate::Data;

pub mod file;
pub mod stdout;

#[cfg(feature = "clickhouse")]
pub mod clickhouse;

/// Exporter
#[async_trait]
pub trait Exporter: Send + Sync + DynClone {
    /// Exports data
    async fn export(&self, data: &Vec<Data>);
}

dyn_clone::clone_trait_object!(Exporter);
