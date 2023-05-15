// Identity processor

use async_trait::async_trait;
use obsv_core::Data;

use super::Processor;

/// Identity processor
#[derive(Debug, Clone)]
pub struct PassThroughProcessor {}

#[async_trait]
impl Processor for PassThroughProcessor {
    async fn process(&mut self, data: Vec<Data>) -> Option<Vec<Data>> {
        log::trace!("id processing");
        Some(data)
    }
}
