//! OTLP logs

use time::OffsetDateTime;

/// Log
pub struct Log {
    /// Timestamp
    pub timestamp: OffsetDateTime,
    /// Content
    pub value: String,
}
