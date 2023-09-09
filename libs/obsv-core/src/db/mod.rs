//! Databases

#[cfg(feature = "clickhouse")]
pub mod clickhouse;

/// Database client
///
/// A DB client can store and retrieve the telemetry data
pub trait DbClient {
    // TODO: add methods
}
