//! Filter processor

use async_trait::async_trait;
use obsv_core::Data;

use super::Processor;

/// Filter processor
#[derive(Debug, Clone)]
pub struct FilterProcessor<F>
where
    F: Fn(&Data) -> bool + Send + Sync + Clone,
{
    /// Filtering rule
    filter: F,
}

impl<F> FilterProcessor<F>
where
    F: Fn(&Data) -> bool + Send + Sync + Clone,
{
    /// Creates a new batch processor
    pub fn new(rule: F) -> Self {
        Self { filter: rule }
    }
}

#[async_trait]
impl<F> Processor for FilterProcessor<F>
where
    F: Fn(&Data) -> bool + Send + Sync + Clone,
{
    async fn process(&mut self, data: Vec<Data>) -> Option<Vec<Data>> {
        log::trace!("filtering processing");
        let mut filtered = Vec::new();
        for d in data {
            if (self.filter)(&d) {
                filtered.push(d);
            }
        }
        return Some(filtered);
    }
}
