//! Monitors

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    /// ID
    pub id: u32,
    /// Name (friendly)
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Kind (eg HTTP, DNS, etc..)
    pub kind: MonitorKind,
    /// Frequency
    pub frequency: Duration,
}

/// Monitor kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitorKind {
    Http { url: String },
    Other { kind: String },
}

impl Monitor {
    /// Creates a new monitor
    pub fn new(id: u32, name: String, kind: MonitorKind) -> Self {
        Self {
            id,
            name,
            description: None,
            kind,
            frequency: Duration::from_secs(5 * 60), // 5 minutes
        }
    }

    /// Creates a new HTTP monitor
    pub fn http(id: u32, name: String, url: String) -> Self {
        Self::new(id, name, MonitorKind::Http { url })
    }

    /// Sets the description
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Sets the frequency
    pub fn frequency(mut self, frequency: Duration) -> Self {
        self.frequency = frequency;
        self
    }
}

/// The result of a monitor check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorCheck {
    /// Monitor ID
    pub monitor_id: u32,
    /// Timestamp (UNIX nanoseconds)
    pub timestamp: i128,
    /// Status
    pub status: MonitorCheckStatus,
}

impl MonitorCheck {
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

/// The status of a monitor check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitorCheckStatus {
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

/// A check which is in progress
#[derive(Debug, Clone)]
pub struct MonitorRunningCheck {
    /// Monitor ID
    pub monitor_id: u32,
    /// Timestamp (UNIX timestamp)
    pub timestamp: i128,
    /// Start
    pub start: Instant,
}

impl Monitor {
    /// Starts a new check
    pub fn start_check(&self) -> MonitorRunningCheck {
        MonitorRunningCheck {
            monitor_id: self.id,
            timestamp: OffsetDateTime::now_utc().unix_timestamp_nanos(),
            start: Instant::now(),
        }
    }
}

impl MonitorRunningCheck {
    /// Marks the check as success
    pub fn ok(self) -> MonitorCheck {
        MonitorCheck {
            monitor_id: self.monitor_id,
            timestamp: self.timestamp,
            status: MonitorCheckStatus::Success {
                resp_time: self.start.elapsed(),
            },
        }
    }

    /// Marks the check as a failure
    pub fn error(&mut self, error: &str) -> MonitorCheck {
        MonitorCheck {
            monitor_id: self.monitor_id,
            timestamp: self.timestamp,
            status: MonitorCheckStatus::Failure {
                resp_time: self.start.elapsed(),
                error: error.to_string(),
            },
        }
    }
}
