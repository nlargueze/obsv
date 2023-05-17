//! OTLP metrics

use obsv_otlp::proto::collector::metrics::v1::ExportMetricsServiceRequest;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    attr::{Attr, Attrs},
    metric::{Metric, Metrics},
};

impl From<ExportMetricsServiceRequest> for Metrics {
    fn from(value: ExportMetricsServiceRequest) -> Self {
        let mut metrics = vec![];
        for resource in value.resource_metrics {
            let resource_attrs = if let Some(r) = resource.resource {
                // NB: the resource defines the
                r.attributes
                    .iter()
                    .map(|kv| Attr::from(kv.clone()))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

            for scope_metrics in resource.scope_metrics {
                let (_scope_name, _scope_version, scope_attrs) =
                    if let Some(scope) = scope_metrics.scope {
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

                for metric in scope_metrics.metrics {
                    let mut metric: Metric = metric.into();
                    metric.add_attrs(resource_attrs.clone());
                    metric.add_attrs(scope_attrs.clone());
                    metrics.push(metric);
                }
            }
        }
        Metrics(metrics)
    }
}

impl From<obsv_otlp::proto::metrics::v1::Metric> for Metric {
    fn from(value: obsv_otlp::proto::metrics::v1::Metric) -> Self {
        let name = value.name;

        // TODO: implement the mapping of description, unit, data
        // let _descr = value.description;
        // let _unit = value.unit;
        // let _data = match value.data {
        //     Some(data) => match data {
        //         obsv_otlp::proto::metrics::v1::metric::Data::Gauge(g) => {
        //             g.data_points.iter().map(|dp| dp.value).collect()
        //         }
        //         obsv_otlp::proto::metrics::v1::metric::Data::Sum(s) => {
        //             s.data_points.iter().map(|dp| dp.value).collect()
        //         }
        //         obsv_otlp::proto::metrics::v1::metric::Data::Histogram(h) => {
        //             h.data_points.iter().map(|dp| dp.value).collect()
        //         }
        //         obsv_otlp::proto::metrics::v1::metric::Data::ExponentialHistogram(e) => {
        //             e.data_points.iter().map(|dp| dp.value).collect()
        //         }
        //         obsv_otlp::proto::metrics::v1::metric::Data::Summary(s) => {
        //             s.data_points.iter().map(|dp| dp.).collect()
        //         }
        //     },
        //     None => {}
        // };

        Metric {
            id: Uuid::new_v4().as_u128(),
            timestamp: OffsetDateTime::now_utc().unix_timestamp_nanos() as u64,
            name,
            attrs: Attrs::new(),
        }
    }
}
