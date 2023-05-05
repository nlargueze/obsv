//! HTTP monitor

use std::time::Duration;

use async_trait::async_trait;
use hyper::{Body, Client, Uri};

use crate::error::Error;

use super::{Monitor, MonitorCheck};

pub use hyper::Method;

/// HTTP monitor
#[derive(Debug)]
pub struct HttpMonitor {
    /// ID
    pub id: String,
    /// Friendly name
    pub name: String,
    /// Frequency
    pub frequency: Duration,
    /// Uri
    pub uri: Uri,
    /// Method
    pub method: Method,
    /// Headers
    pub headers: Vec<(String, String)>,
}

impl From<hyper::http::uri::InvalidUri> for Error {
    fn from(value: hyper::http::uri::InvalidUri) -> Self {
        Error::new(value.to_string().as_str())
    }
}

impl HttpMonitor {
    /// Instantiates a new [HttpMonitor]
    ///
    /// **Defaults**:
    /// - method = GET
    /// - frequency = 1min
    ///
    pub fn new(id: &str, uri: &str) -> Result<Self, Error> {
        let uri = uri.parse()?;
        let method = Method::GET;
        let frequency = Duration::from_secs(60);

        Ok(Self {
            id: id.to_string(),
            name: id.to_string(),
            frequency,
            uri,
            method,
            headers: vec![],
        })
    }

    /// Sets the friendly name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Sets the method
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Sets the frequency
    pub fn frequency(mut self, freq: Duration) -> Self {
        self.frequency = freq;
        self
    }
}

/// A trait to represent a monitor
#[async_trait]
impl Monitor for HttpMonitor {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn frequency(&self) -> Duration {
        self.frequency
    }

    async fn check(&self) -> MonitorCheck {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let mut req_builder = hyper::Request::builder()
            .uri(self.uri.clone())
            .method(self.method.clone());
        for (k, v) in self.headers.iter() {
            req_builder = req_builder.header(k, v);
        }
        let body = Body::empty();
        let req = req_builder.body(body).unwrap();

        let mut check = MonitorCheck::start(&self.id);
        match client.request(req).await {
            Ok(res) => {
                if res.status().is_success() {
                    check.succeeded();
                } else {
                    check.failed("received http code >= 300");
                }
            }
            Err(err) => check.failed(err.to_string().as_str()),
        };

        check
    }
}
