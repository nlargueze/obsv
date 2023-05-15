//! Batch processor

use async_trait::async_trait;
use obsv_core::Data;

use super::Processor;

/// Batch processor
///
/// `N` is the buffer capacity
#[derive(Debug, Clone)]
pub struct BatchProcessor {
    /// Buffer capacity
    capacity: usize,
    /// Buffer
    buffer: Vec<Data>,
}

impl BatchProcessor {
    /// Creates a new batch processor
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: vec![],
        }
    }
}

#[async_trait]
impl Processor for BatchProcessor {
    async fn process(&mut self, mut data: Vec<Data>) -> Option<Vec<Data>> {
        log::trace!("batch processing");
        self.buffer.append(&mut data);
        if self.buffer.len() <= self.capacity {
            return None;
        }
        let overflown = self.buffer.drain(self.capacity..).collect::<Vec<_>>();
        return Some(overflown);
    }
}
