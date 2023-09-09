//! Webhook notification
//!
//! A webhook notification sends a POST request to the endpoint.

use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Body, Method, Url,
};
use serde::Serialize;

use crate::{error::Error, Channel};

/// Webhook channel
#[derive(Debug, Clone)]
pub struct WebhookChannel {
    /// URL
    pub url: Url,
    /// Friendly name
    pub name: String,
    /// Request client
    pub client: reqwest::Client,
}

/// Webhook channel builder
#[derive(Debug)]
pub struct WebhookChannelBuilder {
    /// URL
    pub url: String,
    /// Friendly name
    pub name: String,
    /// Headers
    pub headers: HashMap<String, String>,
}

/// A webhook message
#[derive(Debug, Clone, Default)]
pub struct WebhookMessage {
    /// Headers
    pub headers: HeaderMap,
    /// Body (JSON)
    pub body: String,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::new(format!("{}", value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::new(format!("{}", value))
    }
}

impl WebhookChannel {
    /// Creates a new [WebhookChannelBuilder]
    pub fn builder(url: &str) -> WebhookChannelBuilder {
        WebhookChannelBuilder {
            url: url.to_string(),
            name: "".to_string(),
            headers: HashMap::new(),
        }
    }

    /// Sends a GET request to the origin
    pub async fn get_origin(&self) -> Result<(), Error> {
        let _res = self.client.get(self.url.as_ref()).send().await?;
        Ok(())
    }

    /// Sends a webhook message
    pub async fn post_message(&self, message: WebhookMessage) -> Result<(), Error> {
        let req = self
            .client
            .request(Method::POST, self.url.as_ref())
            .headers(message.headers)
            .header(CONTENT_TYPE.to_string(), "application/json")
            .body(Body::from(message.body))
            .build()?;
        let res = self.client.execute(req).await?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::new("Webhook failed"))
        }
    }
}

impl WebhookChannelBuilder {
    /// Sets the friendly name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Adds a header
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Builds a new [WebhookChannel]
    pub fn build(self) -> Result<WebhookChannel, Error> {
        let url = self
            .url
            .parse::<Url>()
            .map_err(|err| Error::new(err.to_string()))?;
        let headers = self
            .headers
            .into_iter()
            .try_fold(HeaderMap::new(), |mut map, (k, v)| {
                let k = k
                    .parse::<HeaderName>()
                    .map_err(|err| Error::new(err.to_string()))?;
                let v = v
                    .parse::<HeaderValue>()
                    .map_err(|err| Error::new(err.to_string()))?;
                map.insert(k, v);
                Ok(map) as Result<HeaderMap, Error>
            })?;
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(WebhookChannel {
            url,
            name: self.name,
            client,
        })
    }
}

#[async_trait]
impl Channel for WebhookChannel {
    type Message = WebhookMessage;

    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        self.post_message(message).await
    }
}

impl WebhookMessage {
    /// Creates a new [WebHook]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a header
    pub fn header(mut self, key: &str, value: &str) -> Result<Self, Error> {
        let k = key
            .parse::<HeaderName>()
            .map_err(|err| Error::new(err.to_string()))?;
        let v = value
            .parse::<HeaderValue>()
            .map_err(|err| Error::new(err.to_string()))?;
        self.headers.insert(k, v);
        Ok(self)
    }

    /// Adds a body
    pub fn body(mut self, body: impl Serialize) -> Result<Self, Error> {
        self.body = serde_json::to_string(&body)?;
        Ok(self)
    }
}
