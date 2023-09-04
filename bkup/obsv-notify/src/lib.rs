//! This crate provides a notification service

use std::{str::FromStr, time::Duration};

use duration_string::DurationString;
use obsv_message::channel::Channel;
use rule::Rule;

pub mod error;
pub mod rule;

/// Notification service
pub struct NotifService {
    /// Frequency when the service checks for notifications to send
    pub frequency: Duration,
    /// Rules
    pub rules: Vec<Box<dyn Rule>>,
    /// Channels
    pub channels: Vec<Box<dyn Channel>>,
}

impl Default for NotifService {
    fn default() -> Self {
        Self {
            frequency: Duration::from_secs(60),
            rules: vec![],
            channels: vec![],
        }
    }
}

impl NotifService {
    /// Instantiates a new [NotifService]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the frequency
    pub fn frequency(mut self, freq: &str) -> Self {
        self.frequency = DurationString::from_str(freq).unwrap().into();
        self
    }

    /// Adds a [Rule]
    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Adds a [Channel]
    pub fn channel(mut self, channel: impl Channel + 'static) -> Self {
        self.channels.push(Box::new(channel));
        self
    }

    /// Starts the service
    pub async fn start(self) {
        let (tx, _) = tokio::sync::broadcast::channel::<String>(100);

        // task to check for events
        let mut rule_tasks = vec![];
        for rule in self.rules {
            rule_tasks.push(tokio::spawn({
                let tx = tx.clone();
                async move {
                    let mut interval = tokio::time::interval(self.frequency);
                    loop {
                        interval.tick().await;
                        let message = "a service is down";
                        match rule.check().await {
                            Ok(ok) => {
                                if let Some(msg) = ok {
                                    tx.send(msg).unwrap();
                                }
                            }
                            Err(err) => {
                                log::error!("failed to check a rule: {err}");
                            }
                        }
                        tx.send(message.to_string()).unwrap();
                    }
                }
            }));
        }

        // there is a task for each channel waiting to send a notification
        let mut channel_tasks = vec![];
        for channel in self.channels {
            // the message is sent in parallel through each channel
            let mut rx = tx.subscribe();
            channel_tasks.push(tokio::spawn(async move {
                let message = rx.recv().await.unwrap();
                match channel.send(&message).await {
                    Ok(_ok) => {}
                    Err(err) => {
                        log::error!("failed to send message: {message} ({err})");
                    }
                }
            }));
        }

        for task in rule_tasks {
            task.await.unwrap();
        }
        for task in channel_tasks {
            task.await.unwrap();
        }
    }
}
