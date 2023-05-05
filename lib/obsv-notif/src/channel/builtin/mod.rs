//! Builtin notification channels

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "webhook")]
pub mod webhook;
