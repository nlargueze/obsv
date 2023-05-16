//! OTLP metrics

use obsv_otlp::proto::collector::metrics::v1::ExportMetricsServiceRequest;

use crate::{
    attr::Attr,
    metric::{Metric, Metrics},
};

impl From<ExportMetricsServiceRequest> for Metrics {
    fn from(req: ExportMetricsServiceRequest) -> Self {
        let mut metrics = vec![];
        for resource_metric in &req.resource_metrics {
            let resource_attrs = if let Some(r) = &resource_metric.resource {
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

            for scope_metric in &resource_metric.scope_metrics {
                for otlp_metric in &scope_metric.metrics {
                    let mut metric: Metric = otlp_metric.clone().into();
                    metric.add_attrs(resource_attrs.clone());
                    metrics.push(metric);
                }
            }
        }
        Metrics::new(metrics)
    }
}

impl From<obsv_otlp::proto::metrics::v1::Metric> for Metric {
    fn from(_metric: obsv_otlp::proto::metrics::v1::Metric) -> Self {
        // log::trace!("Converting OTLP metric to core metric: {metric:?}");
        todo!("implement metrics conversion");
        // let _name = metric.name;
        // let _desc = metric.description;
        // let _unit = metric.unit;
        // if let Some(d) = metric.data {
        //     match d {
        //         obsv_otlp::proto::metrics::v1::metric::Data::Gauge(_g) => todo!(),
        //         obsv_otlp::proto::metrics::v1::metric::Data::Sum(_s) => todo!(),
        //         obsv_otlp::proto::metrics::v1::metric::Data::Histogram(_h) => todo!(),
        //         obsv_otlp::proto::metrics::v1::metric::Data::ExponentialHistogram(_h) => todo!(),
        //         obsv_otlp::proto::metrics::v1::metric::Data::Summary(_s) => todo!(),
        //     }
        // };

        // Metric {
        //     id: uuid::Uuid::new_v4().as_u128(),
        //     timestamp: todo!(),
        //     value: todo!(),
        //     attrs: todo!(),
        // }
    }
}
