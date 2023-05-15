//! Attributes

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Collection of attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attrs(pub Vec<Attr>);

impl Attrs {
    /// Create a new [Attrs]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an attributes
    pub fn push(&mut self, attr: Attr) {
        self.0.push(attr);
    }
}

impl From<Vec<Attr>> for Attrs {
    fn from(value: Vec<Attr>) -> Self {
        Self(value)
    }
}

/// Attribute
///
/// An attribute is a key-value pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attr {
    /// Key
    pub key: String,
    /// Value
    pub value: AttrValue,
}

impl Attr {
    /// Creates a new [Attr]
    pub fn new(key: &str, value: impl Into<AttrValue>) -> Self {
        Self {
            key: key.to_string(),
            value: value.into(),
        }
    }

    /// Returns the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the value
    pub fn value(&self) -> &AttrValue {
        &self.value
    }
}

/// Attribute value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttrValue {
    None,
    Bool(bool),
    Str(String),
    Uint(u128),
    Int(i128),
    Float(f64),
    Array(Vec<AttrValue>),
    Map(HashMap<String, AttrValue>),
    Bytes(Vec<u8>),
}

impl AttrValue {
    /// Instantiates a new bool value
    pub fn new_bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Instantiates a new string value
    pub fn new_string(value: &str) -> Self {
        Self::Str(value.to_string())
    }

    /// Instantiates a new uint value
    pub fn new_uint(value: impl Into<u128>) -> Self {
        Self::Uint(value.into())
    }

    /// Instantiates a new int value
    pub fn new_int(value: impl Into<i128>) -> Self {
        Self::Int(value.into())
    }

    /// Instantiates a new float value
    pub fn new_float(value: impl Into<f64>) -> Self {
        Self::Float(value.into())
    }

    /// Instantiates a new array value
    pub fn new_array(value: Vec<AttrValue>) -> Self {
        Self::Array(value)
    }

    /// Instantiates a new map value
    pub fn new_map(value: HashMap<String, AttrValue>) -> Self {
        Self::Map(value)
    }

    /// Instantiates a new bytes value
    pub fn new_bytes(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }

    /// Returns the bool value
    pub fn bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the string value
    pub fn string(&self) -> Option<String> {
        match self {
            Self::Str(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Returns the Uint value
    pub fn uint(&self) -> Option<u128> {
        match self {
            Self::Uint(u) => Some(*u),
            _ => None,
        }
    }

    /// Returns the Int value
    pub fn int(&self) -> Option<i128> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the float value
    pub fn float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Returns the array value
    pub fn array(&self) -> Option<Vec<AttrValue>> {
        match self {
            Self::Array(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Returns the map value
    pub fn map(&self) -> Option<HashMap<String, AttrValue>> {
        match self {
            Self::Map(m) => Some(m.clone()),
            _ => None,
        }
    }

    /// Returns the bytes value
    pub fn bytes(&self) -> Option<Vec<u8>> {
        match self {
            Self::Bytes(b) => Some(b.clone()),
            _ => None,
        }
    }
}
