//! OTLP attributes

use crate::attr::{Attr, AttrValue};

impl From<obsv_otlp::proto::common::v1::KeyValue> for Attr {
    fn from(value: obsv_otlp::proto::common::v1::KeyValue) -> Self {
        let key = value.key.clone();
        let value = value.value;
        Attr::new(&key, value)
    }
}

impl From<Option<obsv_otlp::proto::common::v1::AnyValue>> for AttrValue {
    fn from(value: Option<obsv_otlp::proto::common::v1::AnyValue>) -> Self {
        match value {
            Some(v) => v.into(),
            None => AttrValue::None,
        }
    }
}

impl From<obsv_otlp::proto::common::v1::AnyValue> for AttrValue {
    fn from(value: obsv_otlp::proto::common::v1::AnyValue) -> Self {
        match value.value {
            Some(v) => v.into(),
            None => AttrValue::None,
        }
    }
}

impl From<obsv_otlp::proto::common::v1::any_value::Value> for AttrValue {
    fn from(value: obsv_otlp::proto::common::v1::any_value::Value) -> Self {
        match value {
            obsv_otlp::proto::common::v1::any_value::Value::StringValue(s) => AttrValue::Str(s),
            obsv_otlp::proto::common::v1::any_value::Value::BoolValue(b) => AttrValue::Bool(b),
            obsv_otlp::proto::common::v1::any_value::Value::IntValue(i) => AttrValue::Int(i),
            obsv_otlp::proto::common::v1::any_value::Value::DoubleValue(d) => AttrValue::Float(d),
            obsv_otlp::proto::common::v1::any_value::Value::ArrayValue(arr) => {
                AttrValue::Array(arr.values.iter().map(|v| v.clone().into()).collect())
            }
            obsv_otlp::proto::common::v1::any_value::Value::KvlistValue(dict) => AttrValue::Map(
                dict.values
                    .iter()
                    .map(|kv| {
                        let key = kv.key.clone();
                        let value: AttrValue = kv.value.clone().into();
                        (key, value)
                    })
                    .collect(),
            ),
            obsv_otlp::proto::common::v1::any_value::Value::BytesValue(b) => AttrValue::Bytes(b),
        }
    }
}
