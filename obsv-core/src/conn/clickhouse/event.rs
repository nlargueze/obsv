//! DB user events

use clickhouse_client::{orm::prelude::*, query::Where};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{attr::Attrs, UserEvent};

use super::Error;

/// User event in DB
#[derive(Debug, Clone, DbRecord)]
#[db(table = "events_user")]
pub struct DbUserEvent {
    /// ID
    #[db(primary)]
    pub id: Uuid,
    /// Date (ns from EPOCH)
    pub timestamp: OffsetDateTime,
    /// Kind
    pub kind: String,
    /// Message
    pub message: String,
    /// Attributes
    pub attrs: Attrs,
}

impl Default for DbUserEvent {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            timestamp: OffsetDateTime::UNIX_EPOCH,
            kind: String::new(),
            message: String::new(),
            attrs: Attrs::new(),
        }
    }
}

impl From<crate::UserEvent> for DbUserEvent {
    fn from(value: crate::UserEvent) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: value.timestamp,
            kind: value.kind,
            message: value.message,
            attrs: value.attrs,
        }
    }
}

impl From<DbUserEvent> for crate::UserEvent {
    fn from(value: DbUserEvent) -> Self {
        Self {
            timestamp: value.timestamp,
            kind: value.kind,
            message: value.message,
            attrs: value.attrs,
        }
    }
}

impl super::ClickhouseConnector {
    /// Inserts events
    pub async fn insert_user_events(&self, events: &[UserEvent]) -> Result<(), Error> {
        let db_events = events
            .iter()
            .map(|e| DbUserEvent::from(e.clone()))
            .collect::<Vec<_>>();
        Ok(self.client.insert(&db_events).await?)
    }

    /// Returns all the user events
    pub async fn get_user_events(&self, where_cond: Where) -> Result<Vec<UserEvent>, Error> {
        let db_events = self.client.select::<DbUserEvent>(&[], where_cond).await?;
        Ok(db_events
            .into_iter()
            .map(UserEvent::from)
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clickhouse_user_events() {
        let conn = crate::conn::clickhouse::tests::init_clickhouse().await;
        let event = UserEvent {
            timestamp: OffsetDateTime::now_utc(),
            kind: "page_view".to_string(),
            message: "Page has been visited".to_string(),
            attrs: Attrs::new(),
        };

        if let Err(err) = conn.insert_user_events(&[event]).await {
            tracing::error!(
                test = "test_clickhouse_user_events",
                error = err.to_string(),
                "failed to insert"
            );
            panic!("{}", err);
        }
        match conn.get_user_events(Where::empty()).await {
            Ok(events) => {
                tracing::info!(test = "test_clickhouse_user_events", "OK");
                eprintln!("{events:#?}")
            }
            Err(err) => {
                tracing::error!(
                    test = "test_clickhouse_user_events",
                    error = err.to_string(),
                    "failed to select"
                );
                panic!("{}", err);
            }
        };
    }
}
