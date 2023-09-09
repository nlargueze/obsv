//! Scope

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::AttrValue;

/// A scope
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    /// Name
    pub name: String,
    /// Attributes
    pub attrs: HashMap<String, AttrValue>,
}
