//! Batch processor

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::Processor;

/// Batch processor
///
/// `N` is the buffer capacity
#[derive(Debug)]
pub struct BatchProcessor {
    /// Buffer capacity
    capacity: usize,
    /// Buffer
    buffer: Arc<Mutex<Data>>,
}

impl Clone for BatchProcessor {
    fn clone(&self) -> Self {
        Self {
            capacity: self.capacity,
            buffer: self.buffer.clone(),
        }
    }
}

impl BatchProcessor {
    /// Creates a new batch processor
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: Arc::new(Mutex::new(Data::default())),
        }
    }
}

#[async_trait]
impl Processor for BatchProcessor {
    async fn process(&mut self, mut data: Vec<Data>) -> Option<Vec<Data>> {
        log::trace!("batch processing");
        let mut buffer = self.buffer.lock().await;
        *buffer.append(&mut data);
        if *buffer.len() <= self.capacity {
            return None;
        }
        let overflown = self.buffer.drain(self.capacity..).collect::<Vec<_>>();
        return Some(overflown);
    }
}
