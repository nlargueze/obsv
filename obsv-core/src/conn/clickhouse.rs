//! Clickhouse connector

use clickhouse_client::schema::{DbType, DbValue};

use crate::{
    attr::{Attr, AttrValue, Attrs},
    event::{Event, Events},
};

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

impl DbType for Events {
    // NB: an attribute value is stored as a string
    const TYPE: &'static str = "Array(String)";
}

impl DbValue for Events {
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
                ron::from_str::<Event>(event_str).expect("Invalid event db value")
            })
            .collect::<Vec<_>>();
        Ok(Events(events))
    }
}
