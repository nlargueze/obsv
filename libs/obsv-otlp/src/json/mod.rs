//! JSON serialization

use core::fmt;

use serde::{de::Visitor, Deserializer, Serializer};

use crate::proto::collector::{
    logs::v1::ExportLogsPartialSuccess, metrics::v1::ExportMetricsPartialSuccess,
    trace::v1::ExportTracePartialSuccess,
};

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

/// Deserializes the field 'partialSuccess' of the trace response
///
/// The OTLP collector returns a JSON object with a field 'partialSuccess' as '{}',
/// whcih is not recognized as a valid Option<ExportTracePartialSuccess>
pub fn deserialize_trace_partial_success<'de, D>(
    deserializer: D,
) -> Result<Option<ExportTracePartialSuccess>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(TracePartialSuccessVisitor)
}

// Visitor to deserialize a trace partial success
struct TracePartialSuccessVisitor;

impl<'de> Visitor<'de> for TracePartialSuccessVisitor {
    type Value = Option<ExportTracePartialSuccess>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Option<ExportTracePartialSuccess>")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut rejected_spans = 0_i64;
        let mut error_message = String::new();
        while let Some(key) = map.next_key::<&str>().unwrap() {
            match key {
                "rejected_spans" => {
                    rejected_spans = map.next_value::<i64>()?;
                }
                "error_message" => {
                    error_message = map.next_value::<String>()?;
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key,
                        &["rejected_spans", "error_message"],
                    ))
                }
            }
        }

        if rejected_spans == 0 && error_message.is_empty() {
            return Ok(None);
        }

        Ok(Some(ExportTracePartialSuccess {
            rejected_spans,
            error_message,
        }))
    }
}

/// Deserializes the field 'partialSuccess' of the logs esponse
pub fn deserialize_logs_partial_success<'de, D>(
    deserializer: D,
) -> Result<Option<ExportLogsPartialSuccess>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(LogsPartialSuccessVisitor)
}

// Visitor to deserialize a logs partial success
struct LogsPartialSuccessVisitor;

impl<'de> Visitor<'de> for LogsPartialSuccessVisitor {
    type Value = Option<ExportLogsPartialSuccess>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Option<ExportLogsPartialSuccess>")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut rejected_log_records = 0_i64;
        let mut error_message = String::new();
        while let Some(key) = map.next_key::<&str>().unwrap() {
            match key {
                "rejected_log_records" => {
                    rejected_log_records = map.next_value::<i64>()?;
                }
                "error_message" => {
                    error_message = map.next_value::<String>()?;
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key,
                        &["rejected_log_records", "error_message"],
                    ))
                }
            }
        }

        if rejected_log_records == 0 && error_message.is_empty() {
            return Ok(None);
        }

        Ok(Some(ExportLogsPartialSuccess {
            rejected_log_records,
            error_message,
        }))
    }
}

/// Deserializes the field 'partialSuccess' of the metrics esponse
pub fn deserialize_metrics_partial_success<'de, D>(
    deserializer: D,
) -> Result<Option<ExportMetricsPartialSuccess>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(MetricsPartialSuccessVisitor)
}

// Visitor to deserialize a metrics partial success
struct MetricsPartialSuccessVisitor;

impl<'de> Visitor<'de> for MetricsPartialSuccessVisitor {
    type Value = Option<ExportMetricsPartialSuccess>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid Option<ExportMetricsPartialSuccess>")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut rejected_data_points = 0_i64;
        let mut error_message = String::new();
        while let Some(key) = map.next_key::<&str>().unwrap() {
            match key {
                "rejected_data_points" => {
                    rejected_data_points = map.next_value::<i64>()?;
                }
                "error_message" => {
                    error_message = map.next_value::<String>()?;
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key,
                        &["rejected_log_records", "error_message"],
                    ))
                }
            }
        }

        if rejected_data_points == 0 && error_message.is_empty() {
            return Ok(None);
        }

        Ok(Some(ExportMetricsPartialSuccess {
            rejected_data_points,
            error_message,
        }))
    }
}
