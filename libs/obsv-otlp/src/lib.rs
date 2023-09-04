//! Crate for OpenTelemetry
//!
//! # Features
//!
//! # Build script
//!
//! The build script downloads the OTLP proto specs from [Github](https://github.com/open-telemetry/opentelemetry-proto).
//! The script uses the env. variable `OBSV_OTEL_PROTO_VERSION` to set the Otel specs version. It defaults to `v0.19.0`.

pub mod conv;
pub mod json;
pub mod proto;
pub mod server;
