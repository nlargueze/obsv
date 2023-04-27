//! Exporter
//!
//! The exporter is responsible for exporting the received data

/// Exporter
pub trait Exporter {
    /// Exports data
    fn export();
}
