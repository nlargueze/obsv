//! Collector
//!
//! The collector is the server responsible for receiving, processing, and exporting metrics, traces, logs, and other data.
//!
//! The collector provides:
//! - a HTTP API, which mimics the OpenTelemetry API
//! - a gRPC API, which mimics the OpenTelemetry API
//!  
//! # Features
//!
//! - NA

pub mod expt;
pub mod proc;
pub mod recv;
pub mod serv;
