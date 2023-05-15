//! Error

/// Error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);
