//! Email notification channels

use std::time::Duration;

use async_trait::async_trait;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::{channel::Channel, error::Error, Notification};

/// Email channel
#[derive(Debug, Clone)]
pub struct EmailChannel {
    /// Friendly name
    pub name: String,
    /// SMTP client
    pub client: AsyncSmtpTransport<Tokio1Executor>,
    /// From
    pub from: String,
    /// Reply to
    pub reply_to: Option<String>,
    /// To
    pub to: Vec<String>,
    /// CC
    pub cc: Vec<String>,
    /// BCC
    pub bcc: Vec<String>,
    /// Subject
    pub subject: String,
    /// Body
    ///
    /// TODO: improve how to provide a HTML/multipart
    /// read this: https://gist.github.com/tylermakin/d820f65eb3c9dd98d58721c7fb1939a8
    pub body: String,
}

/// Email body
#[derive(Debug, Clone)]
pub enum EmailBody {
    /// Plain text
    Text(String),
    /// HTML
    Html(String),
    /// Text and HTML
    TextAndHtml(String, String),
}

/// Email channel builder
#[derive(Debug)]
pub struct EmailChannelBuilder {
    /// Friendly name
    pub name: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Connection type
    pub conn_type: EmailClientConnType,
    /// Credentials
    pub credentials: Option<(String, String)>,
    /// Timeout
    pub timeout: Option<Duration>,
    /// From
    pub from: String,
    /// To
    pub to: Vec<String>,
    /// Subject
    pub subject: String,
    /// Body
    pub body: String,
    /// Reply to
    pub reply_to: Option<String>,
    /// CC
    pub cc: Vec<String>,
    /// BCC
    pub bcc: Vec<String>,
}

/// Email client connection type
#[derive(Debug, Clone)]
pub enum EmailClientConnType {
    Tls,
    StartTls,
    Insecure,
}

impl EmailChannel {
    /// Instantiates a new builder
    pub fn builder(
        name: &str,
        host: &str,
        port: u16,
        from: &str,
        to: &[&str],
        subject: &str,
        body: &str,
    ) -> EmailChannelBuilder {
        EmailChannelBuilder {
            name: name.to_string(),
            host: host.to_string(),
            port,
            conn_type: EmailClientConnType::Tls,
            credentials: None,
            timeout: None,
            from: from.to_string(),
            to: to.iter().map(|x| x.to_string()).collect(),
            subject: subject.to_string(),
            body: body.to_string(),
            reply_to: None,
            cc: vec![],
            bcc: vec![],
        }
    }
}

impl EmailChannel {
    /// Tests the connection
    pub async fn test_conn(&self) -> Result<bool, Error> {
        Ok(self.client.test_connection().await?)
    }

    /// Sends a message (internal method)
    async fn send_internal(&self, notif: impl Notification) -> Result<(), Error> {
        let email = self.assemble_email(notif)?;
        let _res = self.client.send(email).await?;
        Ok(())
    }

    /// Internal method to get the email
    fn assemble_email(&self, _notif: impl Notification) -> Result<Message, Error> {
        let mut builder = Message::builder();

        builder = builder.from(self.from.parse()?);
        if let Some(a) = &self.reply_to {
            builder = builder.from(a.parse()?);
        };
        for to in &self.to {
            builder = builder.to(to.parse()?);
        }
        for cc in &self.cc {
            builder = builder.cc(cc.parse()?);
        }
        for bcc in &self.bcc {
            builder = builder.bcc(bcc.parse()?);
        }

        builder = builder.subject(self.subject.clone());
        builder = builder.header(ContentType::TEXT_PLAIN);
        // TODO: add notification content to email
        let email = builder.body(self.body.clone())?;

        Ok(email)
    }
}

impl EmailChannelBuilder {
    /// Sets the connection type
    pub fn conn_type(mut self, conn_type: EmailClientConnType) -> Self {
        self.conn_type = conn_type;
        self
    }

    /// Sets the credentials
    pub fn credentials(mut self, username: &str, password: &str) -> Self {
        self.credentials = Some((username.to_string(), password.to_string()));
        self
    }

    /// Sets the timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Adds a CC
    pub fn add_cc(mut self, cc: &str) -> Self {
        self.cc.push(cc.to_string());
        self
    }

    /// Adds a BCC
    pub fn add_bcc(mut self, bcc: &str) -> Self {
        self.bcc.push(bcc.to_string());
        self
    }

    /// Sets a body
    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    /// Sets a reply to
    pub fn reply_to(mut self, to: &str) -> Self {
        self.reply_to = Some(to.to_string());
        self
    }

    /// Creates a new instance from [WebhookConfig]
    pub fn channel(self) -> Result<EmailChannel, Error> {
        let mut transport_builder = match self.conn_type {
            EmailClientConnType::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)?,
            EmailClientConnType::StartTls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.host)?
            }
            EmailClientConnType::Insecure => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.host)
            }
        }
        .port(self.port)
        // .tls(tls)
        // .authentication(mechanisms)
        .timeout(self.timeout);

        if let Some((username, password)) = self.credentials {
            let creds = Credentials::new(username, password);
            transport_builder = transport_builder.credentials(creds);
        }
        let transport = transport_builder.build::<Tokio1Executor>();

        Ok(EmailChannel {
            name: self.name,
            client: transport,
            from: self.from,
            to: self.to,
            reply_to: self.reply_to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            body: self.body,
        })
    }
}

impl From<lettre::error::Error> for Error {
    fn from(value: lettre::error::Error) -> Self {
        Error::InvalidConfig(format!("{}", value))
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        Error::InvalidConfig(format!("{}", value))
    }
}

impl From<lettre::address::AddressError> for Error {
    fn from(value: lettre::address::AddressError) -> Self {
        Error::InvalidConfig(format!("{}", value))
    }
}

#[async_trait]
impl Channel for EmailChannel {
    async fn send(&self, notif: impl Notification + Send) -> Result<(), Error> {
        Ok(self.send_internal(notif).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextNotification;

    fn get_channel() -> EmailChannel {
        // NB: use a SMTP catch server (mailhog, mailpit)
        EmailChannel::builder(
            "test email channel",
            "localhost",
            1025,
            "alice <alice@dummy.com>",
            &["bob <bob@dummy.com>"],
            "test email",
            "test body",
        )
        .conn_type(EmailClientConnType::Insecure)
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
