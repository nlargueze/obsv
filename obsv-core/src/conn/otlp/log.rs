//! OTLP logs

use obsv_otlp::proto::collector::logs::v1::ExportLogsServiceRequest;

use crate::{
    attr::Attr,
    log::{Log, Logs},
    Data,
};

impl From<ExportLogsServiceRequest> for Data {
    fn from(value: ExportLogsServiceRequest) -> Self {
        Data::Logs(value.into())
    }
}

impl From<ExportLogsServiceRequest> for Logs {
    fn from(req: ExportLogsServiceRequest) -> Self {
        let mut logs = vec![];
        for resource_logs in &req.resource_logs {
            let resource_attrs = if let Some(r) = &resource_logs.resource {
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

            for scope_log in &resource_logs.scope_logs {
                for otlp_log in &scope_log.log_records {
                    let mut log: Log = otlp_log.clone().into();
                    log.add_attrs(resource_attrs.clone());
                    logs.push(log);
                }
            }
        }
        Logs::new(logs)
    }
}

impl From<obsv_otlp::proto::logs::v1::LogRecord> for Log {
    fn from(_log: obsv_otlp::proto::logs::v1::LogRecord) -> Self {
        todo!("implement the logs conversion");
        // log::trace!("Converting OTLP metric to core log: {log:?}");
        // let _name = log.trace_id
        // let _desc = metric.description;
        // let _unit = metric.unit;

        // Log {
        //     id: todo!(),
        //     timestamp: todo!(),
        //     message: todo!(),
        //     attrs: todo!(),
        // }
    }
}
