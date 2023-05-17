//! Metrics

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::attr::Attrs;

/// A collection of metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metrics(pub Vec<Metric>);

impl Metrics {
    /// Creates a new collection
    pub fn new(metrics: Vec<Metric>) -> Self {
        Self(metrics)
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

/// A metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Resource name
    pub resource: String,
    /// Resource attributes
    pub resource_attrs: Attrs,
    /// Scope name
    pub scope: String,
    /// Resource attributes
    pub scope_attrs: Attrs,
    /// Name
    pub name: String,
    /// Description
    pub descr: String,
    /// Unit
    pub unit: String,
}
