//! OTLP trace

use std::collections::HashMap;

use obsv_otlp::{conv::ServiceSemConv, proto};
use time::OffsetDateTime;

use crate::{
    attr::{Attr, Attrs},
    trace::{Span, SpanEvent},
    Spans,
};

impl From<proto::collector::trace::v1::ExportTraceServiceRequest> for Spans {
    fn from(value: proto::collector::trace::v1::ExportTraceServiceRequest) -> Spans {
        let mut spans = vec![];
        for resource in value.resource_spans {
            let mut resource_name = "".to_string();
            let resource_attrs = if let Some(r) = resource.resource {
                // NB: the resource defines the
                r.attributes
                    .iter()
                    .map(|kv| {
                        let attr: Attr = kv.clone().into();
                        if attr.key == ServiceSemConv::SERVICE_NAME {
                            resource_name = attr.value.to_string();
                        }
                        (attr.key, attr.value)
                    })
                    .collect::<HashMap<_, _>>()
            } else {
                HashMap::new()
            };

            for scope_spans in resource.scope_spans {
                let (scope_name, _scope_version, scope_attrs) =
                    if let Some(scope) = scope_spans.scope {
                        (
                            scope.name,
                            scope.version,
                            scope
                                .attributes
                                .iter()
                                .map(|kv| {
                                    let attr: Attr = kv.clone().into();
                                    (attr.key, attr.value)
                                })
                                .collect::<HashMap<_, _>>(),
                        )
                    } else {
                        (String::new(), String::new(), HashMap::new())
                    };

                for span in scope_spans.spans {
                    let mut span: Span = span.into();
                    span.resource = resource_name.clone();
                    span.resource_attrs = resource_attrs.clone();
                    span.scope = scope_name.clone();
                    span.scope_attrs = scope_attrs.clone();
                    spans.push(span);
                }
            }
        }
        Spans::new(spans)
    }
}

impl From<proto::trace::v1::Span> for Span {
    fn from(value: proto::trace::v1::Span) -> Self {
        // log::trace!("Converting OTLP span to core span: {span:?}");
        let trace_id = u128::from_be_bytes(value.trace_id.try_into().unwrap_or_else(|bytes| {
            log::error!("invalid trace ID: {:?}", bytes);
            [0; 16]
        }));
        let span_id = u64::from_be_bytes(value.span_id.try_into().unwrap_or_else(|bytes| {
            log::error!("invalid span ID: {:?}", bytes);
            [0; 8]
        }));
        let parent_span_id = if !value.parent_span_id.is_empty() {
            u64::from_be_bytes(value.parent_span_id.try_into().unwrap_or_else(|bytes| {
                log::error!("invalid parent span ID: {:?}", bytes);
                [0; 8]
            }))
        } else {
            0
        };
        let name = value.name;
        let kind = value.kind;
        let start = OffsetDateTime::from_unix_timestamp_nanos(value.start_time_unix_nano.into())
            .unwrap_or_else(|ns| {
                log::error!("invalid start time (ns): {}", ns);
                OffsetDateTime::UNIX_EPOCH
            });
        let end = OffsetDateTime::from_unix_timestamp_nanos(value.end_time_unix_nano.into())
            .unwrap_or_else(|ns| {
                log::error!("invalid start time (ns): {}", ns);
                OffsetDateTime::UNIX_EPOCH
            });
        let attrs = value
            .attributes
            .iter()
            .map(|kv| {
                let attr: Attr = kv.clone().into();
                (attr.key, attr.value)
            })
            .collect::<HashMap<_, _>>()
            .into();
        let events = value
            .events
            .iter()
            .map(|ev| ev.clone().into())
            .collect::<Vec<_>>()
            .into();

        Span {
            resource: String::new(),
            resource_attrs: Attrs::new(),
            scope: String::new(),
            scope_attrs: Attrs::new(),
            trace_id,
            span_id: span_id.into(),
            parent_span_id: parent_span_id.into(),
            name,
            kind,
            start,
            end,
            attrs,
            events,
        }
    }
}

impl From<proto::trace::v1::span::Event> for SpanEvent {
    fn from(value: proto::trace::v1::span::Event) -> Self {
        let timestamp = OffsetDateTime::from_unix_timestamp_nanos(value.time_unix_nano.into())
            .unwrap_or_else(|ns| {
                log::error!("invalid timestamp (ns): {}", ns);
                OffsetDateTime::UNIX_EPOCH
            });

        let attrs = value
            .attributes
            .iter()
            .map(|kv| {
                let attr: Attr = kv.clone().into();
                (attr.key, attr.value)
            })
            .collect();

        SpanEvent {
            timestamp,
            name: value.name,
            attrs,
        }
    }
}
