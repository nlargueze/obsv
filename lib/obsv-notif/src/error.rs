//! Error

/// A channel error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid channel configuration
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    /// Channel connection error
    #[error("connection error: {0}")]
    ConnectErr(String),
    /// Failed request
    #[error("failed request: {0}")]
    FailedRequest(String),
}
