//! OpenTelemetry connector

use obsv_otlp::proto::{
    collector::trace::v1::ExportTraceServiceRequest,
    common::v1::{any_value::Value, AnyValue, KeyValue},
};

pub use obsv_otlp::*;
use uuid::Uuid;

use crate::{
    attr::{Attr, AttrValue},
    event::Event,
    trace::{Span, Spans},
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

impl From<obsv_otlp::proto::trace::v1::span::Event> for Event {
    fn from(event: obsv_otlp::proto::trace::v1::span::Event) -> Self {
        let attrs = event
            .attributes
            .iter()
            .map(|kv| kv.clone().into())
            .collect::<Vec<Attr>>()
            .into();

        Event {
            id: Uuid::new_v4().as_u128(),
            timestamp: event.time_unix_nano,
            kind: "na".to_string(),
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

// /// Metric kind
// pub enum MetricKind {
//     /// A value that accumulates over time – you can think of this like an odometer on a car; it only ever goes up.
//     Counter,
//     /// Same as the Counter, but is collected once for each export. Could be used if you don’t have access to the continuous increments, but only to the aggregated value.
//     AsyncCounter,
//     /// A value that accumulates over time, but can also go down again. An example could be a queue length, it will increase and decrease with the number of work items in the queue.
//     UpDownCounter,
//     /// Same as the UpDownCounter, but is collected once for each export. Could be used if you don’t have access to the continuous changes, but only to the aggregated value (e.g., current queue size).
//     AsyncUpDownCounter,
//     /// Measures a current value at the time it is read. An example would be the fuel gauge in a vehicle. Gauges are always asynchronous.
//     Gauge,
//     /// A histogram is a client-side aggregation of values, e.g., request latencies. A histogram is likely a good choice if you have a lot of values, and are not interested in every individual value, but a statistic about these values (e.g., How many requests take fewer than 1s?)
//     Histogram,
// }
