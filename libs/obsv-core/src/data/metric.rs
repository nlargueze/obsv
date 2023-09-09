//! Metrics

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{AttrValue, Scope, Service};

/// A collection of metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsData {
    /// Metrics
    pub metrics: Vec<ServiceMetrics>,
}

/// A collection of metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// Service
    pub service: Service,
    /// Scope
    pub scope: Option<Scope>,
    /// Metrics
    pub metrics: Vec<Metric>,
}

/// A metric
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    /// Name
    pub name: String,
    /// Description
    pub descr: String,
    /// Unit description
    pub unit: String,
    /// Value
    pub value: String,
    /// Attributes
    pub attrs: HashMap<String, AttrValue>,
}
