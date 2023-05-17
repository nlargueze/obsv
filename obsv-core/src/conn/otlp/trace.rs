//! OTLP trace

use crate::{
    attr::Attr,
    trace::{Span, SpanEvent, Spans},
};

impl From<obsv_otlp::proto::collector::trace::v1::ExportTraceServiceRequest> for Spans {
    fn from(value: obsv_otlp::proto::collector::trace::v1::ExportTraceServiceRequest) -> Self {
        let mut spans = vec![];
        for resource in value.resource_spans {
            let resource_attrs = if let Some(r) = resource.resource {
                // NB: the resource defines the
                r.attributes
                    .iter()
                    .map(|kv| Attr::from(kv.clone()))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            for scope_spans in resource.scope_spans {
                let (_scope_name, _scope_version, scope_attrs) =
                    if let Some(scope) = scope_spans.scope {
                        (
                            scope.name,
                            scope.version,
                            scope
                                .attributes
                                .iter()
                                .map(|kv| Attr::from(kv.clone()))
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        (String::new(), String::new(), vec![])
                    };

                for span in scope_spans.spans {
                    let mut span: Span = span.into();
                    span.add_attrs(resource_attrs.clone());
                    span.add_attrs(scope_attrs.clone());
                    spans.push(span);
                }
            }
        }
        Spans(spans)
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
