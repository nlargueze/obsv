//! Clickhouse connector

use async_trait::async_trait;
use clickhouse_client::{
    http::Client,
    schema::{DbRowExt, DbType, DbValue, Schema},
};

use crate::{
    attr::{Attr, AttrValue, Attrs},
    event::Event,
    log::Log,
    metric::Metric,
    monitor::Monitor,
    trace::{Span, SpanEvent, SpanEvents},
};

#[derive(Debug, thiserror::Error)]
#[error("Clickhouse error: {0}")]
pub struct Error(String);

/// Extension for the clickhouse client
#[async_trait]
pub trait ClientObsvExt {
    /// Returns the DB schema for obversability
    fn obsv_schema(db: &str) -> Schema {
        let tbl_events = Event::db_schema();
        let tbl_traces = Span::db_schema();
        let tbl_metrics = Metric::db_schema();
        let tbl_logs = Log::db_schema();
        let tbl_monitors = Monitor::db_schema();
        Schema::new(db)
            .table(tbl_traces)
            .table(tbl_metrics)
            .table(tbl_logs)
            .table(tbl_events)
            .table(tbl_monitors)
    }

    /// Creates the clickhouse database and tables
    async fn init_obsv_db(&self) -> Result<(), Error>;
}

#[async_trait]
impl ClientObsvExt for Client {
    async fn init_obsv_db(&self) -> Result<(), Error> {
        let db = match &self.db {
            Some(db) => db,
            None => return Err(Error("client does not have a DB".to_string())),
        };
        let schema = Self::obsv_schema(db);
        self.create_db(&schema)
            .await
            .map_err(|err| Error(err.to_string()))?;
        for table in &schema.tables {
            self.create_table(table)
                .await
                .map_err(|err| Error(err.to_string()))?;
        }
        Ok(())
    }
}

impl DbType for Attrs {
    // NB: an attribute value is stored as a string
    const TYPE: &'static str = "Map(String, String)";
}

impl DbValue for Attrs {
    // NB: we store the attribute value with the RON serializer
    fn to_sql_str(&self) -> String {
        format!(
            "{{{}}}",
            self.0
                .iter()
                .map(|attr| {
                    let key = &attr.key;
                    let value_str = match ron::to_string(&attr.value) {
                        Ok(ok) => ok,
                        Err(err) => {
                            panic!("Failed to serialize attr value: {}", err);
                        }
                    };
                    format!("'{}': {}", key, value_str)
                })
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    fn from_sql_str(s: &str) -> Result<Self, String> {
        let s = match s.strip_prefix('{') {
            Some(s) => s,
            None => return Err("Invalid map".to_string()),
        };
        let s = match s.strip_suffix('}') {
            Some(s) => s,
            None => return Err("Invalid map".to_string()),
        };
        let attrs = s
            .split(',')
            .map(|part| {
                // item is 'key1':1
                match part.split_once(':') {
                    Some((key, v)) => {
                        let v = v.trim();
                        let attr_value =
                            ron::from_str::<AttrValue>(v).expect("Invalid attribute db value");
                        Ok(Attr::new(key, attr_value))
                    }
                    None => Err("Invalid map".to_string()),
                }
            })
            .collect::<Result<Vec<Attr>, String>>()?;
        Ok(Attrs(attrs))
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::OnceCell;

    static ONCE: OnceCell<Client> = OnceCell::const_new();

    async fn init() -> &'static Client {
        crate::tests::init_tracer();
        ONCE.get_or_init(|| async {
            let client = Client::new("http://localhost:8123").database("obsv");
            client.init_obsv_db().await.unwrap();
            client
        })
        .await
    }

    #[tokio::test]
    async fn test_clickhouse_init() {
        let _client = init().await;
    }
}
