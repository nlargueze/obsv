//! Core structures and utilities for `obsv`

use serde::{Deserialize, Serialize};

pub mod attr;
pub mod conn;
pub mod event;
pub mod log;
pub mod metric;
pub mod monitor;
pub mod trace;

/// A core piece of data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Spans(trace::Spans),
    /// Log
    Log(log::Log),
    // /// Metric
    // Metric(metric::Metric),
    /// Event
    Event(event::Event),
}
