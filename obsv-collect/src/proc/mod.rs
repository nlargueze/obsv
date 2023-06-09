//! Processor
//!
//! The processor is responsible for processing received data

use async_trait::async_trait;
use dyn_clone::DynClone;

use crate::Data;

pub mod batch;
pub mod filter;
pub mod id;

/// Processor
#[async_trait]
pub trait Processor: Send + Sync + DynClone {
    /// Processes the data
    ///
    /// The data is returned once processed, or None if the data processed latter
    async fn process(&mut self, data: Vec<Data>) -> Option<Vec<Data>>;
}

dyn_clone::clone_trait_object!(Processor);
