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
