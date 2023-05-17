//! DB Traces

use clickhouse_client::{orm::prelude::*, query::Where};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{Span, SpanEvent};

use super::{attr::DbAttrs, Error};

/// A DB span
#[derive(Debug, Clone, DbRecord)]
#[db(table = "traces")]
pub struct DbSpan {
    /// ID
    #[db(primary)]
    pub id: Uuid,
    /// Resource name
    pub resource: String,
    /// Resource attributes
    pub resource_attrs: DbAttrs,
    /// Scope name
    pub scope: String,
    /// Resource attributes
    pub scope_attrs: DbAttrs,
    /// Trace ID
    pub trace_id: u128,
    /// Span ID
    pub span_id: u64,
    /// Parent span ID (0 if no parent)
    pub parent_span_id: u64,
    /// Span name
    pub name: String,
    /// Kind
    pub kind: i32,
    /// Start time (ns from EPOCH)
    pub start: OffsetDateTime,
    /// End time (ns from EPOCH)
    pub end: OffsetDateTime,
    /// Attributes
    pub attrs: DbAttrs,
    /// Events
    pub events: DbSpanEvents,
}

impl Default for DbSpan {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            resource: String::default(),
            resource_attrs: DbAttrs::default(),
            scope: String::default(),
            scope_attrs: DbAttrs::default(),
            trace_id: 0,
            span_id: 0,
            parent_span_id: 0,
            start: OffsetDateTime::UNIX_EPOCH,
            end: OffsetDateTime::UNIX_EPOCH,
            name: String::default(),
            kind: 0,
            attrs: DbAttrs::default(),
            events: DbSpanEvents::default(),
        }
    }
}

impl From<crate::Span> for DbSpan {
    fn from(value: crate::Span) -> Self {
        Self {
            id: Uuid::new_v4(),
            resource: value.resource,
            resource_attrs: value.resource_attrs.into(),
            scope: value.scope,
            scope_attrs: value.scope_attrs.into(),
            trace_id: value.trace_id,
            span_id: value.span_id,
            parent_span_id: value.parent_span_id,
            name: value.name,
            kind: value.kind,
            start: value.start,
            end: value.end,
            attrs: value.attrs.into(),
            events: value.events.into(),
        }
    }
}

impl From<DbSpan> for crate::Span {
    fn from(value: DbSpan) -> Self {
        Self {
            resource: value.resource,
            resource_attrs: value.resource_attrs.into(),
            scope: value.scope,
            scope_attrs: value.scope_attrs.into(),
            trace_id: value.trace_id,
            span_id: value.span_id,
            parent_span_id: value.parent_span_id,
            name: value.name,
            kind: value.kind,
            start: value.start,
            end: value.end,
            attrs: value.attrs.into(),
            events: value.events.into(),
        }
    }
}

/// DB span events
#[derive(Debug, Clone, Default)]
pub struct DbSpanEvents(Vec<SpanEvent>);

impl From<Vec<SpanEvent>> for DbSpanEvents {
    fn from(value: Vec<SpanEvent>) -> Self {
        Self(value)
    }
}

impl From<DbSpanEvents> for Vec<SpanEvent> {
    fn from(value: DbSpanEvents) -> Self {
        value.0
    }
}

impl DbType for DbSpanEvents {
    // NB: an attribute value is stored as a string
    const TYPE: &'static str = "Array(String)";
}

impl DbValue for DbSpanEvents {
    // NB: we store the attribute value with the RON serializer
    fn to_sql_str(&self) -> String {
        format!(
            "[{}]",
            self.0
                .iter()
                .map(|event| {
                    match ron::to_string(event) {
                        Ok(event_str) => event_str,
                        Err(err) => {
                            panic!("Failed to serialize attr value: {}", err);
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn from_sql_str(s: &str) -> Result<Self, String> {
        let s = match s.strip_prefix('[') {
            Some(s) => s,
            None => return Err("Invalid event array".to_string()),
        };
        let s = match s.strip_suffix(']') {
            Some(s) => s,
            None => return Err("Invalid event array".to_string()),
        };
        let events = s
            .split(',')
            .map(|event_str| {
                let event_str = event_str.trim();
                ron::from_str::<SpanEvent>(event_str)
                    .unwrap_or_else(|_| panic!("Invalid event db value: {event_str}"))
            })
            .collect::<Vec<_>>();
        Ok(events.into())
    }
}

impl super::ClickhouseConnector {
    /// Inserts spans
    pub async fn insert_spans(&self, spans: &[Span]) -> Result<(), Error> {
        let db_spans = spans
            .iter()
            .map(|m| DbSpan::from(m.clone()))
            .collect::<Vec<_>>();
        Ok(self.client.insert(&db_spans).await?)
    }

    /// Returns the spans
    pub async fn get_spans(&self, where_cond: Where) -> Result<Vec<Span>, Error> {
        let db_spans = self.client.select::<DbSpan>(&[], where_cond).await?;
        Ok(db_spans.into_iter().map(Span::from).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use time::OffsetDateTime;

    use super::*;
    use crate::{attr::Attrs, Span};

    #[tokio::test]
    async fn test_clickhouse_spans() {
        let conn = crate::conn::clickhouse::tests::init_clickhouse().await;
        let span = Span {
            resource: "test_service".into(),
            resource_attrs: HashMap::new(),
            scope: "test_scope".into(),
            scope_attrs: HashMap::new(),
            trace_id: 1,
            span_id: 1,
            parent_span_id: 0,
            name: "test_span".into(),
            kind: 0,
            start: OffsetDateTime::now_utc(),
            end: OffsetDateTime::now_utc(),
            attrs: Attrs::new(),
            events: vec![],
        };

        if let Err(err) = conn.insert_spans(&[span]).await {
            tracing::error!(
                test = "test_clickhouse_spans",
                error = err.to_string(),
                "failed to insert"
            );
            panic!("{}", err);
        }
        match conn.get_spans(Where::empty()).await {
            Ok(spans) => {
                tracing::info!(test = "test_clickhouse_spans", "OK");
                eprintln!("{spans:#?}")
            }
            Err(err) => {
                tracing::error!(
                    test = "test_clickhouse_spans",
                    error = err.to_string(),
                    "failed to get"
                );
                panic!("{}", err);
            }
        };
    }
}
