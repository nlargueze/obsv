//! Value

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// An attribute value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttrValue {
    None,
    Bool(bool),
    Uint(u64),
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<AttrValue>),
    Map(HashMap<String, AttrValue>),
}
