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

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error(value.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error(value.to_string())
    }
}

impl From<hyper::http::method::InvalidMethod> for Error {
    fn from(value: hyper::http::method::InvalidMethod) -> Self {
        Error(value.to_string())
    }
}
