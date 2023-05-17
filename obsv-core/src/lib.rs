//! Core structures and utilities for `obsv`

mod attr;
mod event;
mod log;
mod metric;
mod monitor;
mod trace;

pub mod conn;

pub use self::event::*;
pub use self::log::*;
pub use self::metric::*;
pub use self::monitor::*;
pub use self::trace::*;

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
                .events_only(true)
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
