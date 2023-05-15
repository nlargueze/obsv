//! Metrics

use serde::{Deserialize, Serialize};

#[cfg(feature = "clickhouse")]
use clickhouse_client::schema::prelude::*;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::{Attr, Attrs};

/// Metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics(Vec<Metric>);

impl std::fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, span) in self.0.iter().enumerate() {
            write!(f, "{span}")?;
            if i < self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Metrics {
    /// Creates a new collection
    pub fn new(metrics: Vec<Metric>) -> Self {
        Self(metrics)
    }
}

/// A metric
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "clickhouse", derive(DbRow))]
#[cfg_attr(feature = "clickhouse", db(table = "metrics"))]
pub struct Metric {
    /// ID
    #[cfg_attr(feature = "clickhouse", db(primary))]
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Value
    pub value: f64,
    /// Attributes
    pub attrs: Attrs,
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            id: 0,
            timestamp: 0,
            value: 0.0,
            attrs: Attrs::new(),
        }
    }
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = OffsetDateTime::from_unix_timestamp_nanos(self.timestamp as i128)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH);

        write!(
            f,
            "[{}] [{}] {}",
            dt.format(&Rfc3339).unwrap(),
            self.id,
            self.value
        )
    }
}

impl Metric {
    /// Adds an attribute
    pub fn add_attr(&mut self, attr: impl Into<Attr>) -> &mut Self {
        self.attrs.push(attr.into());
        self
    }

    /// Adds attributes
    pub fn add_attrs(&mut self, attrs: impl IntoIterator<Item = impl Into<Attr>>) -> &mut Self {
        for attr in attrs.into_iter() {
            self.attrs.push(attr.into());
        }
        self
    }
}
