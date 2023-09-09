//! HTTP

use std::{collections::HashMap, str::FromStr};

use crate::error::Error;

/// Trace parent header
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TraceParentHeader {
    /// Version
    pub version: u8,
    /// Trace ID (16 bytes)
    pub trace_id: u128,
    /// Parent ID (8 bytes)
    pub parent_id: u64,
    /// 8-bit flags
    pub flags: u8,
}

impl TraceParentHeader {
    /// Header name
    pub const NAME: &str = "traceparent";
}

impl std::fmt::Display for TraceParentHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}-{}-{}-{:02}",
            hex::encode([self.version]),
            hex::encode(self.trace_id.to_be_bytes()),
            hex::encode(self.parent_id.to_be_bytes()),
            hex::encode([self.flags]),
        )
    }
}

impl FromStr for TraceParentHeader {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(Error::string(format!("invalid traceparent header: {}", s)));
        }
        let version: [u8; 1] = hex::decode(parts[0])?
            .try_into()
            .map_err(|_| Error::new("invalid traceparent header version"))?;
        let trace_id: [u8; 16] = hex::decode(parts[1])?
            .try_into()
            .map_err(|_| Error::new("invalid traceparent header trace_id"))?;
        let parent_id: [u8; 8] = hex::decode(parts[2])?
            .try_into()
            .map_err(|_| Error::new("invalid traceparent header parent_id"))?;
        let flags: [u8; 1] = hex::decode(parts[3])?
            .try_into()
            .map_err(|_| Error::new("invalid traceparent header flags"))?;
        Ok(Self {
            version: version[0],
            trace_id: u128::from_be_bytes(trace_id),
            parent_id: u64::from_be_bytes(parent_id),
            flags: flags[0],
        })
    }
}

/// Trace state header
///
/// A trace state header contains a list of key/value pairs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceStateHeader {
    /// Value
    ///
    /// Identifiers MUST begin with a lowercase letter or a digit,
    /// and can only contain lowercase letters (a-z), digits (0-9),
    /// underscores (_), dashes (-), asterisks (*), and forward slashes (/).
    ///
    /// The value is an opaque string containing up to 256 printable ASCII [RFC0020] characters
    /// (i.e., the range 0x20 to 0x7E) except comma (,) and (=).
    /// Note that this also excludes tabs, newlines, carriage returns, etc.
    pub values: HashMap<String, String>,
}

impl TraceStateHeader {
    /// Header name
    pub const NAME: &str = "tracestate";
}

impl std::fmt::Display for TraceStateHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl FromStr for TraceStateHeader {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for kv_str in s.split(',') {
            let parts = kv_str.split('=').collect::<Vec<_>>();
            match parts.len() {
                2 => {
                    let key = parts[0].to_string();
                    let value = parts[1].to_string();
                    map.insert(key, value);
                }
                _ => {
                    return Err(Error::new("invalid tracestate header"));
                }
            }
        }
        Ok(Self { values: map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_trace_parent_header() {
        let header = TraceParentHeader {
            version: 0,
            trace_id: 1,
            parent_id: 1,
            flags: 1,
        };
        let header_str = header.to_string();
        assert_eq!(
            header_str,
            "00-00000000000000000000000000000001-0000000000000001-01"
        );

        let header_parsed = header_str.parse::<TraceParentHeader>().unwrap();
        assert_eq!(header_parsed, header);
    }

    #[test]
    fn http_trace_state_header() {
        let header = TraceStateHeader {
            values: HashMap::from([
                ("key1".to_string(), "abc".to_string()),
                ("key2".to_string(), "abcd".to_string()),
            ]),
        };
        let header_str = header.to_string();
        assert_eq!(header_str, "key1=abc,key2=abcd");

        let header_parsed = header_str.parse::<TraceStateHeader>().unwrap();
        assert_eq!(header_parsed, header);
    }
}
