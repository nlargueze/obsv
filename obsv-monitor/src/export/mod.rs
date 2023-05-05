//! Export

use async_trait::async_trait;

use crate::{error::Error, monitor::MonitorCheck};

pub mod stdout;

/// A trait to represent a monitor exporter
#[async_trait]
pub trait Exporter: Send + Sync {
    /// Returns the exporter id
    fn id(&self) -> String;

    /// Export a monitor check
    async fn export(&self, check: &MonitorCheck) -> Result<(), Error>;
}
