//! Monitors

use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    /// Unique ID
    pub id: String,
    /// Friendly name
    pub name: String,
    /// Kind (eg HTTP, DNS, etc..)
    pub kind: String,
    /// Target (eg. service, url, etc..)
    pub target: String,
    /// Frequency
    pub frequency: Duration,
}

/// The result of a monitor check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorCheck {
    /// Monitor ID
    pub monitor_id: String,
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Status
    pub status: MonitorCheckStatus,
}

/// The status of a monitor check
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MonitorCheckStatus {
    /// Initialized
    #[default]
    Init,
    /// Success
    Success {
        /// Response time
        resp_time: Duration,
    },
    /// Failure
    Failure {
        /// Response time
        resp_time: Duration,
        /// Error message
        error: String,
    },
}

impl Monitor {
    /// Starts a new check
    pub fn start_check(&self) -> MonitorCheck {
        MonitorCheck {
            monitor_id: self.id.clone(),
            timestamp: OffsetDateTime::now_utc(),
            status: MonitorCheckStatus::Init,
        }
    }
}

impl MonitorCheck {
    /// Marks the check as success
    pub fn succeeded(&mut self) {
        self.status = MonitorCheckStatus::Success {
            resp_time: (OffsetDateTime::now_utc() - self.timestamp)
                .try_into()
                .unwrap(),
        };
    }

    /// Marks the check as a failure
    pub fn failed(&mut self, error: &str) {
        self.status = MonitorCheckStatus::Failure {
            resp_time: (OffsetDateTime::now_utc() - self.timestamp)
                .try_into()
                .unwrap(),
            error: error.to_string(),
        }
    }

    /// Checks if the check is a success
    pub fn is_success(&self) -> bool {
        matches!(self.status, MonitorCheckStatus::Success { resp_time: _ })
    }

    /// Checks if the check is an error and returns the error message
    pub fn is_failure(&self) -> Option<&str> {
        match &self.status {
            MonitorCheckStatus::Failure {
                resp_time: _,
                error,
            } => Some(error),
            _ => None,
        }
    }
}
