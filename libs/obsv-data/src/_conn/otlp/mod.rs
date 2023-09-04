//! OpenTelemetry connector

mod attr;
mod log;
mod metric;
mod trace;

/// Re-export of OTLP specs
pub use obsv_otlp::*;
