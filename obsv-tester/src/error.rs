//! Error

/// Error
#[derive(Debug)]
pub enum Error {
    /// Invalid request
    InvalidRequest(Box<dyn std::error::Error>),
    /// Invalid response
    InvalidResponse(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequest(e) => write!(f, "{}", e),
            Error::InvalidResponse(e) => write!(f, "{}", e),
        }
    }
}

/// A simple adhoc error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct AdhocError(pub String);
