//! OTLP trace

use obsv_otlp::proto::{
    collector::trace::v1::ExportTraceServiceRequest,
    common::v1::{any_value::Value, AnyValue, KeyValue},
};

use crate::{
    attr::{Attr, AttrValue},
    trace::{Span, SpanEvent, Spans},
    Data,
};

impl From<ExportTraceServiceRequest> for Data {
    fn from(value: ExportTraceServiceRequest) -> Self {
        Data::Spans(value.into())
    }
}

impl From<ExportTraceServiceRequest> for Spans {
    fn from(req: ExportTraceServiceRequest) -> Self {
        let mut spans = vec![];
        for resource_span in &req.resource_spans {
            let resource_attrs = if let Some(r) = &resource_span.resource {
                r.attributes
                    .iter()
                    .map(|kv| {
                        let attr: Attr = kv.clone().into();
                        attr
                    })
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            for scope_span in &resource_span.scope_spans {
                for otlp_span in &scope_span.spans {
                    let mut span: Span = otlp_span.clone().into();
                    span.add_attrs(resource_attrs.clone());
                    spans.push(span);
                }
            }
        }
        Spans::new(spans)
    }
}

impl From<obsv_otlp::proto::trace::v1::Span> for Span {
    fn from(span: obsv_otlp::proto::trace::v1::Span) -> Self {
        // log::trace!("Converting OTLP span to core span: {span:?}");
        let trace_id = u128::from_be_bytes(span.trace_id.try_into().unwrap());
        let span_id = u64::from_be_bytes(span.span_id.try_into().unwrap());
        let parent_span_id = if !span.parent_span_id.is_empty() {
            u64::from_be_bytes(span.parent_span_id.try_into().unwrap())
        } else {
            0
        };
        let name = span.name;
        let start = span.start_time_unix_nano;
        let end = span.end_time_unix_nano;
        let attrs = span
            .attributes
            .iter()
            .map(|kv| kv.clone().into())
            .collect::<Vec<_>>()
            .into();
        let events = span
            .events
            .iter()
            .map(|ev| ev.clone().into())
            .collect::<Vec<_>>()
            .into();

        Span {
            trace_id,
            id: span_id.into(),
            parent_id: parent_span_id.into(),
            name,
            start,
            end,
            attrs,
            events,
        }
    }
}

impl From<obsv_otlp::proto::trace::v1::span::Event> for SpanEvent {
    fn from(event: obsv_otlp::proto::trace::v1::span::Event) -> Self {
        let attrs = event
            .attributes
            .iter()
            .map(|kv| kv.clone().into())
            .collect::<Vec<Attr>>()
            .into();

        SpanEvent {
            timestamp: event.time_unix_nano,
            name: event.name,
            message: "Event".to_string(),
            attrs,
        }
    }
}

impl From<Value> for AttrValue {
    fn from(value: Value) -> Self {
        match value {
            obsv_otlp::proto::common::v1::any_value::Value::StringValue(s) => AttrValue::Str(s),
            obsv_otlp::proto::common::v1::any_value::Value::BoolValue(b) => AttrValue::Bool(b),
            obsv_otlp::proto::common::v1::any_value::Value::IntValue(i) => AttrValue::Int(i.into()),
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

impl From<AnyValue> for AttrValue {
    fn from(value: AnyValue) -> Self {
        match value.value {
            Some(v) => v.into(),
            None => AttrValue::None,
        }
    }
}

impl From<Option<AnyValue>> for AttrValue {
    fn from(value: Option<AnyValue>) -> Self {
        match value {
            Some(v) => v.into(),
            None => AttrValue::None,
        }
    }
}

impl From<KeyValue> for Attr {
    fn from(kv: KeyValue) -> Self {
        let key = kv.key.clone();
        let value = kv.value;
        Attr::new(&key, value)
    }
}
