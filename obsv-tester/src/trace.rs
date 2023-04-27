//! Trace generation

/// Generates a single OpenTelemetry trace
// #[tracing::instrument]
fn _generate() {
    // tracing::info!("Generating trace");
    _my_service_fn()
}

/// Send traces with different levels
// #[tracing::instrument]
fn _my_service_fn() {
    // tracing::trace!("**trace**");
    // tracing::debug!("**debug**");
    // tracing::info!("**info**");
    // tracing::warn!("**warn**");
    // tracing::error!("**error**");
}
