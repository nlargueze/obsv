//! OTLP logs

use uuid::Uuid;

use crate::{
    attr::{Attr, AttrValue},
    log::{Log, Logs},
};

impl From<obsv_otlp::proto::collector::logs::v1::ExportLogsServiceRequest> for Logs {
    fn from(value: obsv_otlp::proto::collector::logs::v1::ExportLogsServiceRequest) -> Self {
        let mut logs = vec![];
        for resource in value.resource_logs {
            let resource_attrs = if let Some(r) = resource.resource {
                // NB: the resource defines the
                r.attributes
                    .iter()
                    .map(|kv| Attr::from(kv.clone()))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            for scope_logs in resource.scope_logs {
                let (_scope_name, _scope_version, scope_attrs) =
                    if let Some(scope) = scope_logs.scope {
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

                for record in scope_logs.log_records {
                    let mut log: Log = record.into();
                    log.add_attrs(resource_attrs.clone());
                    log.add_attrs(scope_attrs.clone());
                    logs.push(log);
                }
            }
        }
        Logs(logs)
    }
}

impl From<obsv_otlp::proto::logs::v1::LogRecord> for Log {
    fn from(value: obsv_otlp::proto::logs::v1::LogRecord) -> Self {
        let ts = value.time_unix_nano;
        let _ts_observed = value.observed_time_unix_nano;
        let severity = value.severity_number;
        let _severity_txt = value.severity_text;
        let body = AttrValue::from(value.body);
        let attrs = value
            .attributes
            .iter()
            .map(|kv| Attr::from(kv.clone()))
            .collect::<Vec<_>>();
        let _flags = value.flags;
        let trace_id = value.trace_id;
        let span_id = value.span_id;

        Log {
            id: Uuid::new_v4().as_u128(),
            trace_id: u128::from_be_bytes(trace_id.try_into().unwrap_or_default()),
            span_id: u64::from_be_bytes(span_id.try_into().unwrap_or_default()),
            timestamp: ts,
            level: severity,
            message: body.to_string(),
            attrs: attrs.into(),
        }
    }
}
