//! JSON serialization

use serde::Serializer;

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
