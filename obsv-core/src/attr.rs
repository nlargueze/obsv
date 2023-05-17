//! Attributes

use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

/// Collection of attributes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attrs(pub Vec<Attr>);

impl Attrs {
    /// Create a new [Attrs]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new attributes
    pub fn attr(mut self, attr: Attr) -> Self {
        self.0.push(attr);
        self
    }

    /// Adds a new attributes
    pub fn push(&mut self, attr: Attr) {
        self.0.push(attr);
    }
}

impl Deref for Attrs {
    type Target = Vec<Attr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Attrs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Attr>> for Attrs {
    fn from(value: Vec<Attr>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Attrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|attr| format!("{attr}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{s}")
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

impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

/// Attribute value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttrValue {
    None,
    Bool(bool),
    Uint(u64),
    Int(i64),
    Float(f64),
    Str(String),
    Bytes(Vec<u8>),
    Array(Vec<AttrValue>),
    Map(HashMap<String, AttrValue>),
}

impl AttrValue {
    /// Creates a new [AttrValue]
    pub fn new(value: impl Into<Self>) -> Self {
        value.into()
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
    pub fn uint(&self) -> Option<u64> {
        match self {
            Self::Uint(u) => Some(*u),
            _ => None,
        }
    }

    /// Returns the Int value
    pub fn int(&self) -> Option<i64> {
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

/// Implements
macro_rules! impl_attr_value {
    ($TY:ty, $VAR:tt) => {
        impl From<$TY> for AttrValue {
            fn from(value: $TY) -> Self {
                AttrValue::$VAR(value.into())
            }
        }
    };
}

impl_attr_value!(bool, Bool);
impl_attr_value!(u8, Uint);
impl_attr_value!(u16, Uint);
impl_attr_value!(u32, Uint);
impl_attr_value!(u64, Uint);
impl_attr_value!(i8, Int);
impl_attr_value!(i16, Int);
impl_attr_value!(i32, Int);
impl_attr_value!(i64, Int);
impl_attr_value!(f32, Float);
impl_attr_value!(f64, Float);
impl_attr_value!(String, Str);
impl_attr_value!(&String, Str);
impl_attr_value!(&str, Str);
impl_attr_value!(Vec<u8>, Bytes);
impl_attr_value!(Vec<AttrValue>, Array);
impl_attr_value!(HashMap<String, AttrValue>, Map);

impl std::fmt::Display for AttrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrValue::None => write!(f, "NULL"),
            AttrValue::Bool(b) => write!(f, "{b}"),
            AttrValue::Str(s) => write!(f, "{s}"),
            AttrValue::Uint(u) => write!(f, "{u}"),
            AttrValue::Int(i) => write!(f, "{i}"),
            AttrValue::Float(x) => write!(f, "{x}"),
            AttrValue::Array(arr) => {
                let s = arr
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{s}")
            }
            AttrValue::Map(map) => map
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .fmt(f),
            AttrValue::Bytes(bytes) => write!(f, "{bytes:?}"),
        }
    }
}
