//! Core features
//!
//! This crates contains the core elements for observability

/// Log
#[derive(Debug)]
pub struct Log {}

/// Trace
#[derive(Debug)]
pub struct Trace {}

/// Metric
#[derive(Debug)]
pub struct Metric {}

// Attributes
// net.transport	IP.TCP
// net.peer.ip	10.244.0.1
// net.peer.port	10243
// net.host.name	localhost
// http.method	GET
// http.target	/cart
// http.server_name	frontend
// http.route	/cart
// http.scheme	http
// http.host	localhost
// http.flavor	1.1
// http.status_code	200
// http.user_agent	Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36
