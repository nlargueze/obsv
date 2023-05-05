//! Receiver

/// Receiver
pub trait Receiver {
    /// Starts receiving metrics/traces/logs/etc data
    fn start();
}
