//! OTLP logs

use obsv_otlp::proto::collector::logs::v1::ExportLogsServiceRequest;

use crate::{
    attr::Attr,
    log::{Log, Logs},
};

impl From<ExportLogsServiceRequest> for Logs {
    fn from(value: ExportLogsServiceRequest) -> Self {
        let mut logs = vec![];
        for resource in value.resource_logs {
            let resource_attrs = if let Some(r) = resource.resource {
                // NB: the resource defines the
                r.attributes
                    .iter()
                    .map(|kv| Attr::from(*kv))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            for scope_logs in resource.scope_logs {
                let (scope_name, scope_version, scope_attrs) = if let Some(scope) = scope_logs.scope
                {
                    (scope.name, scope.version, scope.attributes)
                } else {
                    (String::new(), String::new(), vec![])
                };

                for record in scope_logs.log_records {
                    let mut log: Log = record.into();
                    log.add_attrs(resource_attrs.clone());
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
        let ts_observed = value.observed_time_unix_nano;
        let severity = value.severity_number;
        let severity_txt = value.severity_text;
        let body = value.body;
        let attrs = value.attributes;
        let flags = value.flags;
        let trace_id = value.trace_id;
        let span_id = value.span_id;

        // Log {
        //     id: todo!(),
        //     timestamp: todo!(),
        //     message: todo!(),
        //     attrs: todo!(),
        // }
    }
}
