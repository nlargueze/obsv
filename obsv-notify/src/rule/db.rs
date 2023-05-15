//! DB rules

use async_trait::async_trait;

use crate::error::Error;

use super::Rule;

/// Rule to check for failed monitors in the db
#[derive(Debug, Default)]
pub struct CheckForMonitorsRule {}

impl CheckForMonitorsRule {
    /// Creates a new [CheckForMonitorsRule]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Rule for CheckForMonitorsRule {
    async fn check(&self) -> Result<Option<String>, Error> {
        // send a DB query
        todo!("DB check for monitors");
    }
}
