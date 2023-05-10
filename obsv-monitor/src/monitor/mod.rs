//! Monitors

use std::{fmt::Display, time::Duration};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg(feature = "http")]
pub mod http;

/// A trait to represent a monitor
#[async_trait]
pub trait Monitor: Send + Sync {
    /// Returns the monitor ID
    fn id(&self) -> String;

    /// Returns the monitor friendly name
    fn name(&self) -> String;

    /// Returns the monitor frequency
    fn frequency(&self) -> Duration;

    /// Performs a check
    async fn check(&self) -> MonitorCheck;
}

/// A monitor check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorCheck {
    /// Monitor ID
    pub monitor: String,
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Status
    pub status: MonitorCheckStatus,
}

/// Monitor check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitorCheckStatus {
    Started,
    Success { duration: Duration },
    Failure { duration: Duration, message: String },
}

impl Display for MonitorCheckStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitorCheckStatus::Started => write!(f, "running"),
            MonitorCheckStatus::Success { duration } => write!(f, "OK ({:?})", duration),
            MonitorCheckStatus::Failure { duration, message } => {
                write!(f, "ERROR ({:?}) - {} ", duration, message)
            }
        }
    }
}

impl MonitorCheck {
    /// Instantiates a new [MonitorCheck]
    pub fn start(monitor: &str) -> Self {
        Self {
            monitor: monitor.to_string(),
            timestamp: OffsetDateTime::now_utc(),
            status: MonitorCheckStatus::Started,
        }
    }

    /// Marks the check as success
    pub fn succeeded(&mut self) {
        let duration = OffsetDateTime::now_utc() - self.timestamp;
        self.status = MonitorCheckStatus::Success {
            duration: duration.try_into().unwrap(),
        };
    }

    /// Marks the check as a failure
    pub fn failed(&mut self, msg: &str) {
        let duration = OffsetDateTime::now_utc() - self.timestamp;
        self.status = MonitorCheckStatus::Failure {
            duration: duration.try_into().unwrap(),
            message: msg.to_string(),
        }
    }

    /// Checks if the check is a success
    pub fn is_success(&self) -> bool {
        matches!(self.status, MonitorCheckStatus::Success { duration: _ })
    }

    /// Checks if the check is an error
    pub fn is_error(&self) -> bool {
        matches!(
            self.status,
            MonitorCheckStatus::Failure {
                duration: _,
                message: _
            }
        )
    }
}
