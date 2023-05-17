//! DB Traces

use clickhouse_client::orm::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{attr::Attrs, SpanEvent, SpanEvents};

/// A DB span
#[derive(Debug, Clone, Serialize, Deserialize, DbRecord)]
#[db(table = "traces")]
pub struct DbSpan {
    /// Span ID
    #[db(primary)]
    pub id: u128,
    /// Trace ID
    pub trace_id: u128,
    /// Parent span ID (0 if no parent)
    pub parent_id: u128,
    /// Span name
    pub name: String,
    /// Start time (ns from EPOCH)
    pub start: u64,
    /// End time (ns from EPOCH)
    pub end: u64,
    /// Attributes
    pub attrs: Attrs,
    /// Events
    pub events: SpanEvents,
}

impl Default for DbSpan {
    fn default() -> Self {
        Self {
            id: 0,
            trace_id: 0,
            parent_id: 0,
            start: 0,
            end: 0,
            name: String::new(),
            attrs: Attrs::new(),
            events: SpanEvents::new(),
        }
    }
}

impl From<crate::Span> for DbSpan {
    fn from(value: crate::Span) -> Self {
        Self {
            id: value.id,
            trace_id: value.trace_id,
            parent_id: value.parent_id,
            name: value.name,
            start: value.start,
            end: value.end,
            attrs: value.attrs,
            events: value.events,
        }
    }
}

impl From<DbSpan> for crate::Span {
    fn from(value: DbSpan) -> Self {
        Self {
            id: value.id,
            trace_id: value.trace_id,
            parent_id: value.parent_id,
            name: value.name,
            start: value.start,
            end: value.end,
            attrs: value.attrs,
            events: value.events,
        }
    }
}

impl DbType for SpanEvents {
    // NB: an attribute value is stored as a string
    const TYPE: &'static str = "Array(String)";
}

impl DbValue for SpanEvents {
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
                ron::from_str::<SpanEvent>(event_str).expect("Invalid event db value")
            })
            .collect::<Vec<_>>();
        Ok(SpanEvents(events))
    }
}
