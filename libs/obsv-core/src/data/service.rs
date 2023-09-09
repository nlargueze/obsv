//! Service

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::AttrValue;

/// A service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: HashMap<String, AttrValue>,
}
