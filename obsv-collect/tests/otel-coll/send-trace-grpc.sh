#!/bin/zsh

SCRIPTPATH=$(dirname "$0")
TEST_FILE="$SCRIPTPATH/data/span.json"

# for gRPC reflection
# grpcurl -plaintext :4317 list

grpcurl -d @ :4317 opentelemetry.proto.collector.trace.v1.TraceService/Export < $TEST_FILE
if [ $? -eq 0 ]; then
    echo "OK: test trace sent to gRPC receiver"
else
    echo "!!! ERROR !!!"
fi