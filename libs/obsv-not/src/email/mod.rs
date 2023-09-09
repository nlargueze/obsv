//! Email notification channels

use std::time::Duration;

use async_trait::async_trait;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, Body, MessageBuilder},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use mrml::mjml::MJML;
use serde::Serialize;

use crate::{error::Error, Channel};

impl From<lettre::error::Error> for Error {
    fn from(value: lettre::error::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl From<lettre::address::AddressError> for Error {
    fn from(value: lettre::address::AddressError) -> Self {
        Error::new(value.to_string())
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(value: handlebars::TemplateError) -> Self {
        Error::new(value.to_string())
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(value: handlebars::RenderError) -> Self {
        Error::new(value.to_string())
    }
}

impl From<mrml::prelude::parse::Error> for Error {
    fn from(value: mrml::prelude::parse::Error) -> Self {
        Error::new(value.to_string())
    }
}

impl From<mrml::prelude::render::Error> for Error {
    fn from(value: mrml::prelude::render::Error) -> Self {
        Error::new(value.to_string())
    }
}

/// Email channel
#[derive(Debug, Clone)]
pub struct EmailChannel {
    /// Friendly name
    pub name: String,
    /// SMTP client
    pub client: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailChannel {
    /// Instantiates a new builder
    pub fn builder() -> EmailChannelBuilder {
        EmailChannelBuilder {
            name: "Email channel".to_string(),
            host: "localhost".to_string(),
            port: 1025,
            conn_type: EmailChannelConnType::Insecure,
            credentials: None,
            timeout: None,
        }
    }

    /// Tests the connection
    pub async fn test_conn(&self) -> Result<bool, Error> {
        Ok(self.client.test_connection().await?)
    }

    /// Sends an email
    async fn send_email(&self, email: Email) -> Result<(), Error> {
        let _res = self.client.send(email.into()).await?;
        Ok(())
    }
}

#[async_trait]
impl Channel for EmailChannel {
    type Message = Email;

    async fn send(&self, message: Self::Message) -> Result<(), Error> {
        Ok(self.send_email(message).await?)
    }
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
    pub conn_type: EmailChannelConnType,
    /// Credentials
    pub credentials: Option<(String, String)>,
    /// Timeout
    pub timeout: Option<Duration>,
}

/// Email client connection type
#[derive(Debug, Clone)]
pub enum EmailChannelConnType {
    Tls,
    StartTls,
    Insecure,
}

impl EmailChannelBuilder {
    /// Sets the channel name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Sets the channel host
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Sets the channel port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the connection type
    pub fn conn_type(mut self, conn_type: EmailChannelConnType) -> Self {
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

    /// Creates a new [EmailChannel]
    pub fn build(self) -> Result<EmailChannel, Error> {
        let mut transport_builder = match self.conn_type {
            EmailChannelConnType::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)?,
            EmailChannelConnType::StartTls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.host)?
            }
            EmailChannelConnType::Insecure => {
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
        })
    }
}

/// Email message
#[derive(Debug, Clone)]
pub struct Email {
    /// Email message
    message: lettre::Message,
}

impl Email {
    /// Creates an [EmailBuilder]
    pub fn builder() -> EmailBuilder {
        EmailBuilder {
            builder: MessageBuilder::new(),
            body: Body::new("".to_string()),
        }
    }
}

impl From<Email> for lettre::Message {
    fn from(value: Email) -> Self {
        value.message
    }
}

/// Email builder
#[derive(Debug, Clone)]
pub struct EmailBuilder {
    /// Builder
    builder: MessageBuilder,
    /// Body
    body: Body,
}

impl EmailBuilder {
    /// Sets the from field
    pub fn from(mut self, from: &str) -> Result<Self, Error> {
        self.builder = self.builder.from(from.parse()?);
        Ok(self)
    }

    /// Sets the reply_to field
    pub fn reply_to(mut self, reply_to: &str) -> Result<Self, Error> {
        self.builder = self.builder.reply_to(reply_to.parse()?);
        Ok(self)
    }

    /// Sets (or adds) a to field
    pub fn to(mut self, to: &str) -> Result<Self, Error> {
        self.builder = self.builder.to(to.parse()?);
        Ok(self)
    }

    /// Sets (or adds) a cc field
    pub fn cc(mut self, cc: &str) -> Result<Self, Error> {
        self.builder = self.builder.cc(cc.parse()?);
        Ok(self)
    }

    /// Sets the subject field
    pub fn subject(mut self, subject: &str) -> Self {
        self.builder = self.builder.subject(subject);
        self
    }

    /// Sets the body as text
    pub fn body_text(mut self, body: &str) -> Result<Self, Error> {
        self.builder = self.builder.header(ContentType::TEXT_PLAIN);
        self.body = Body::new(body.to_string());
        Ok(self)
    }

    /// Sets the body as HTML
    pub fn body_html(mut self, body: &str) -> Result<Self, Error> {
        self.builder = self.builder.header(ContentType::TEXT_HTML);
        self.body = Body::new(body.to_string());
        Ok(self)
    }

    /// Sets the body as HTML with a MJML + Handlebars template
    pub fn body_template(
        mut self,
        template: &EmailTemplate,
        data: impl Serialize,
    ) -> Result<Self, Error> {
        let body = template.render(data)?;
        self.builder = self.builder.header(ContentType::TEXT_HTML);
        self.body = Body::new(body);
        Ok(self)
    }

    /// Builds the message
    pub fn build(self) -> Result<Email, Error> {
        Ok(Email {
            message: self.builder.body(self.body)?,
        })
    }
}

/// An email template
///
/// The email template is a MJML template, interplated with handlebars
#[derive(Debug)]
pub struct EmailTemplate<'a> {
    /// handlebars registry
    pub registry: Handlebars<'a>,
}

impl<'a> EmailTemplate<'a> {
    /// Template ID
    const TEMPLATE_ID: &str = "__template__";

    /// Creates a new template
    pub fn new(input: &str) -> Result<Self, Error> {
        // parse the handlebars template
        let mut registry = Handlebars::new();
        registry.register_template_string(Self::TEMPLATE_ID, input)?;
        // validate the MJML template
        let mjml_str = registry.render(Self::TEMPLATE_ID, &())?;
        let _mjml = MJML::parse(mjml_str)?;
        Ok(Self { registry })
    }

    /// Renders the template
    pub fn render(&self, data: impl Serialize) -> Result<String, Error> {
        let mjml_str = self.registry.render(Self::TEMPLATE_ID, &data)?;
        let mjml = MJML::parse(mjml_str)?;
        let opts = mrml::prelude::render::Options::default();
        let body = mjml.render(&opts)?;
        Ok(body)
    }
}
