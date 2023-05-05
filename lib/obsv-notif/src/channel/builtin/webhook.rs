//! Webhook notification
//!
//! A webhook notification sends a POST reques to the endpoint.
//!
//! A payload can be passed to the request, which can be text, json, or binary.
//! In case of text or json payload, the content is interpolated to replace the tokens by the notification values:
//! - `{{event}}`
//! - `{{message}}`

use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::{header::HeaderValue, Body};

use crate::channel::{Channel, Error, Notification};

/// Webhook channel
///
/// A webhook can send a text, JSON, or binary message.
///
/// The string `{{message}}` is interpolated to the notification message.
#[derive(Debug, Clone)]
pub struct WebhookChannel {
    /// Friendly name
    pub name: String,
    /// URL
    pub url: String,
    /// Request headers
    pub headers: Option<HashMap<String, String>>,
    /// Request body
    pub payload: Option<WebhookPayload>,
    /// Request client
    pub client: reqwest::Client,
}

#[derive(Debug)]
pub struct WebhookChannelBuilder {
    /// Friendly name
    pub name: String,
    /// URL
    pub url: String,
    /// Request headers
    pub headers: Option<HashMap<String, String>>,
    /// Request body
    pub payload: Option<WebhookPayload>,
}

/// Payload type
#[derive(Debug, Clone)]
pub enum WebhookPayload {
    /// UTF-8 text
    Text(String),
    /// JSON UTF-8
    Json(String),
    /// Binary
    Binary(Vec<u8>),
}

impl WebhookChannelBuilder {
    /// Sets a new header
    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        if let Some(mut headers) = self.headers {
            headers.insert(name.to_string(), value.to_string());
            self.headers = Some(headers);
        } else {
            let mut m = HashMap::new();
            m.insert(name.to_string(), value.to_string());
            self.headers = Some(m);
        }

        self
    }

    /// Sets a text payload
    pub fn with_body_txt(self, data: &str) -> Self {
        let mut c = self.with_header("Content-Type", "text/plain");
        c.payload = Some(WebhookPayload::Text(data.to_string()));
        c
    }

    /// Sets a json payload
    pub fn with_body_json(self, data: impl serde::Serialize) -> Self {
        let mut c = self.with_header("Content-Type", "application/json");
        let data_json = serde_json::to_string(&data).unwrap();
        c.payload = Some(WebhookPayload::Json(data_json));
        c
    }

    /// Sets a binary payload
    pub fn with_body_bin(self, data: String) -> Self {
        let mut c = self.with_header("Content-Type", "application/octet-stream");
        c.payload = Some(WebhookPayload::Binary(data.as_bytes().to_vec()));
        c
    }

    /// Creates a new instance from [WebhookConfig]
    pub fn channel(self) -> Result<WebhookChannel, Error> {
        let name = self.name;
        let url = self.url;
        let headers = self.headers;
        let payload = self.payload;

        let user_agent = HeaderValue::from_str("obsv-notif").unwrap();
        let client = match reqwest::Client::builder().user_agent(user_agent).build() {
            Ok(c) => c,
            Err(err) => return Err(Error::InvalidConfig(format!("{}", err))),
        };

        Ok(WebhookChannel {
            name,
            url,
            headers,
            payload,
            client,
        })
    }
}

impl WebhookChannel {
    /// Instantiates a new builder
    pub fn builder(name: &str, url: &str) -> WebhookChannelBuilder {
        WebhookChannelBuilder {
            name: name.to_string(),
            url: url.to_string(),
            headers: None,
            payload: None,
        }
    }

    /// Tests the client connection
    ///
    /// It sends a GET request to the URL and checks if any response is received
    pub async fn test_conn(&self) -> Result<(), Error> {
        match self.client.get(&self.url).send().await {
            Ok(_res) => Ok(()),
            Err(err) => Err(Error::ConnectErr(format!("{}", err))),
        }
    }
}

#[async_trait]
impl Channel for WebhookChannel {
    async fn send(&self, notif: impl Notification + Send) -> Result<(), Error> {
        // for text or JSON payload, pass the event and message to the body
        let body = if let Some(payload) = &self.payload {
            match payload {
                WebhookPayload::Text(v) => {
                    let v = v.replace("{{message}}", &notif.message());
                    Body::from(v)
                }
                WebhookPayload::Json(v) => {
                    let v = v.replace("{{message}}", &notif.message());
                    Body::from(v)
                }
                WebhookPayload::Binary(v) => Body::from(v.clone()),
            }
        } else {
            Body::from(vec![])
        };

        match self.client.post(&self.url).body(body).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::FailedRequest(format!(
                        "received code {}",
                        res.status()
                    )))
                }
            }
            Err(err) => Err(Error::FailedRequest(format!("{}", err))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextNotification;

    fn get_channel() -> WebhookChannel {
        // NB: use a dummy server to reply to any request
        WebhookChannel::builder("test-webhook", "http://localhost:3002")
            .with_body_txt("{{event}}: {{message}}")
            .channel()
            .unwrap()
    }

    #[tokio::test]
    async fn test_channel_conn() {
        let channel = get_channel();
        channel.test_conn().await.unwrap();
    }

    #[tokio::test]
    async fn test_notif() {
        let channel = get_channel();
        let notif = TextNotification::new("test-message");
        channel.send(notif).await.unwrap();
    }
}
