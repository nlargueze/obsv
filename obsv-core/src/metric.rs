//! Metrics

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::attr::{Attr, Attrs};

/// A metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// ID
    pub id: u128,
    /// Date (ns from EPOCH)
    pub timestamp: u64,
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: Attrs,
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

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = OffsetDateTime::from_unix_timestamp_nanos(self.timestamp as i128)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH);

        write!(f, "[{}] [{}]", dt.format(&Rfc3339).unwrap(), self.id,)
    }
}

/// A collection of metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metrics(pub Vec<Metric>);

impl Metrics {
    /// Creates a new collection
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for Metrics {
    type Target = Vec<Metric>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Metrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
