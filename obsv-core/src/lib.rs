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
    Logs(log::Logs),
    /// Metric
    Metrics(metric::Metrics),
    /// Event
    Event(event::Event),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Spans(spans) => write!(f, "{spans}"),
            Data::Metrics(metrics) => write!(f, "{metrics}"),
            Data::Logs(logs) => write!(f, "{logs}"),
            Data::Event(event) => write!(f, "[EVENT] {event}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use tracing_ext::sub::PrettyConsoleLayer;
    use tracing_subscriber::{prelude::*, EnvFilter};

    static INIT: Once = Once::new();

    /// Initializes the tracer
    pub(crate) fn init_tracer() {
        INIT.call_once(|| {
            let layer_filter = EnvFilter::from_default_env();
            let layer_console = PrettyConsoleLayer::default()
                .wrapped(true)
                .oneline(false)
                .events_only(false)
                .show_time(false)
                .show_target(true)
                .show_span_info(false)
                .indent(6);
            tracing_subscriber::registry()
                .with(layer_console)
                .with(layer_filter)
                .init();
        });
    }
}
