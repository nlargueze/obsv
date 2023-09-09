//! Email tests
//!
//! This test requites a dummy SMPT server (eg. https://github.com/axllent/mailpit)

use obsv_not::{
    email::{Email, EmailChannel, EmailChannelConnType, EmailTemplate},
    error::Error,
    GenericChannel,
};
use serde::Serialize;

#[tokio::test]
async fn test_email() {
    let channel = EmailChannel::builder()
        .name("email")
        .conn_type(EmailChannelConnType::Insecure)
        .host("localhost")
        .port(1025)
        .build()
        .unwrap();
    let message = Message {
        message: "test".to_string(),
    };
    channel.send(message).await.unwrap();
}

/// Message
#[derive(Debug, Serialize)]
struct Message {
    message: String,
}

/// Template string
static TEMPLATE_STR: &str = include_str!("data/email.hbs");

impl TryFrom<Message> for Email {
    type Error = Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let template = EmailTemplate::new(TEMPLATE_STR)?;
        Email::builder()
            .from("alice <alice@dummy.com>")?
            .to("bob <bob@dummy.com>")?
            .subject("subject")
            .body_template(&template, value)?
            .build()
    }
}
