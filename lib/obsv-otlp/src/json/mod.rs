//! JSON serialization

use core::fmt;

use serde::{de::Visitor, Deserializer, Serializer};

#[cfg(test)]
mod tests;

/// Serializes an ID
///
/// The ID is a 16-bytes (trace ID) or 8-bytes (span ID) array.
/// The Otel collector accepts it as a string in the format "<hex>",
/// with no leading `0x`.
pub fn serialize_id<S>(x: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let is_null = x.iter().fold(true, |is_null, e| *e == 0 && is_null);
    if is_null {
        s.serialize_none()
    } else {
        s.serialize_str(&hex::encode(x))
    }
}

/// Deserializes an ID
pub fn deserialize_id<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    // use our visitor to deserialize an `ActualValue`
    deserializer.deserialize_any(JsonIdVisitor)
}

// Visitor to deseriliaze an ID
struct JsonIdVisitor;

impl<'de> Visitor<'de> for JsonIdVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid ID (bytes or string)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        hex::decode(v).map_err(|err| E::custom(err))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![])
    }
}
