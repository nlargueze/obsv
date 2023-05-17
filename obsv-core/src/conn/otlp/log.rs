//! OTLP logs

use std::collections::HashMap;

use obsv_otlp::{conv::ServiceSemConv, proto};
use time::OffsetDateTime;

use crate::{
    attr::{Attr, AttrValue},
    log::{Log, Logs},
};

impl From<proto::collector::logs::v1::ExportLogsServiceRequest> for Logs {
    fn from(value: obsv_otlp::proto::collector::logs::v1::ExportLogsServiceRequest) -> Self {
        let mut logs = vec![];
        for resource in value.resource_logs {
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

            for scope_logs in resource.scope_logs {
                let (scope_name, _scope_version, scope_attrs) =
                    if let Some(scope) = scope_logs.scope {
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

                for record in scope_logs.log_records {
                    let mut log: Log = record.into();
                    log.resource = resource_name.clone();
                    log.resource_attrs = resource_attrs.clone();
                    log.scope = scope_name.clone();
                    log.scope_attrs = scope_attrs.clone();
                    logs.push(log);
                }
            }
        }
        Logs(logs)
    }
}

impl From<obsv_otlp::proto::logs::v1::LogRecord> for Log {
    fn from(value: obsv_otlp::proto::logs::v1::LogRecord) -> Self {
        let timestamp = OffsetDateTime::from_unix_timestamp_nanos(value.time_unix_nano.into())
            .unwrap_or_else(|ns| {
                log::error!("invalid log time (ns): {}", ns);
                OffsetDateTime::UNIX_EPOCH
            });
        let _ts_observed = value.observed_time_unix_nano;
        let severity = value.severity_number;
        let _severity_txt = value.severity_text;
        let body = AttrValue::from(value.body);
        let attrs = value
            .attributes
            .iter()
            .map(|kv| {
                let attr: Attr = kv.clone().into();
                (attr.key, attr.value)
            })
            .collect::<HashMap<_, _>>();
        let _flags = value.flags;
        let trace_id = u128::from_be_bytes(value.trace_id.try_into().unwrap_or_else(|bytes| {
            log::error!("invalid trace ID: {:?}", bytes);
            [0; 16]
        }));
        let span_id = u64::from_be_bytes(value.span_id.try_into().unwrap_or_else(|bytes| {
            log::error!("invalid span ID: {:?}", bytes);
            [0; 8]
        }));

        Log {
            resource: String::new(),
            resource_attrs: HashMap::new(),
            scope: String::new(),
            scope_attrs: HashMap::new(),
            timestamp,
            level: severity,
            message: body.to_string(),
            attrs,
            trace_id,
            span_id,
        }
    }
}
