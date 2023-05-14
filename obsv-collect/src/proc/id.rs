// Identity processor

use async_trait::async_trait;
use obsv_core::Data;

use super::Processor;

/// Identity processor
#[derive(Debug, Clone)]
pub struct PassThroughProcessor {}

#[async_trait]
impl Processor for PassThroughProcessor {
    fn id(&self) -> String {
        "processor_id".to_string()
    }

    async fn process(&self, data: Data) -> Data {
        log::trace!("[{}] Processing data: {data:?}", self.id());
        data
    }
}
