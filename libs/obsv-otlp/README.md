# obsv-otlp

Support crate for OpenTelemetry.

## OpenTelemetry protocol (OTLP)

The OTLP is implemented over 2 transports (HTTP, GRPC), and both use the same protobuf schema (the schema is defined in [Github](https://github.com/open-telemetry/opentelemetry-proto)).

OTLP/HTTP uses the POST method, the payload either in binary or JSON format, and may use HTTP/1.1 or HTTP/2 transports. The JSON format is defined [here](https://protobuf.dev/programming-guides/proto3/#json).

OTLP/gRPC sends telemetry data with unary requests in ExportTraceServiceRequest for traces, ExportMetricsServiceRequest for metrics, ExportLogsServiceRequest for logs.
