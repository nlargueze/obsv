//! Error

/// Error
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    /// Message
    pub message: String,
}

impl Error {
    /// Creates a new [Error]
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    /// Creates a new [Error] with a string
    pub fn string(message: String) -> Self {
        Self { message }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::new(&value.to_string())
    }
}
