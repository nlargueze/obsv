//! Protobuf definitions for OpenTelemetry.

#[cfg(test)]
mod tests;

// Include the `items` module, which is generated from items.proto.
pub mod collector {
    pub mod logs {
        pub mod v1 {
            tonic::include_proto!("opentelemetry.proto.collector.logs.v1");
        }
    }

    pub mod metrics {
        pub mod v1 {
            tonic::include_proto!("opentelemetry.proto.collector.metrics.v1");
        }
    }

    pub mod trace {
        pub mod v1 {
            tonic::include_proto!("opentelemetry.proto.collector.trace.v1");
        }
    }
}

pub mod common {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.common.v1");
    }
}

pub mod logs {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.logs.v1");
    }
}

pub mod metrics {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.metrics.v1");
    }
}

pub mod resource {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.resource.v1");
    }
}

pub mod trace {
    pub mod v1 {
        tonic::include_proto!("opentelemetry.proto.trace.v1");
    }
}
