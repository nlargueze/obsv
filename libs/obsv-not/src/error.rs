//! Error

/// Messaging error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Message
    pub message: String,
}

impl Error {
    /// Create a new error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
