//! Error

/// Monitor error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);

impl Error {
    /// Creates a new [Error]
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}
