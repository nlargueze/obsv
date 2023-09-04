//! OTLP metrics

use std::collections::HashMap;

use obsv_otlp::{conv::ServiceSemConv, proto};

use crate::{
    attr::{Attr, Attrs},
    metric::{Metric, Metrics},
};

impl From<proto::collector::metrics::v1::ExportMetricsServiceRequest> for Metrics {
    fn from(value: proto::collector::metrics::v1::ExportMetricsServiceRequest) -> Self {
        let mut metrics = vec![];
        for resource in value.resource_metrics {
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

            for scope_metrics in resource.scope_metrics {
                let (scope_name, _scope_version, scope_attrs) =
                    if let Some(scope) = scope_metrics.scope {
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

                for metric in scope_metrics.metrics {
                    let mut metric: Metric = metric.into();
                    metric.resource = resource_name.clone();
                    metric.resource_attrs = resource_attrs.clone();
                    metric.scope = scope_name.clone();
                    metric.scope_attrs = scope_attrs.clone();
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
        let descr = value.description;
        let unit = value.unit;

        // TODO: add metric data
        // let _data =  match value.data {
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
        //             s.data_points.iter().map(|dp| dp).collect()
        //         }
        //     },
        //     None => {}
        // };

        Metric {
            resource: String::new(),
            resource_attrs: Attrs::new(),
            scope: String::new(),
            scope_attrs: Attrs::new(),
            name,
            descr,
            unit,
        }
    }
}
