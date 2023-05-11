#!/bin/zsh

SCRIPTPATH=$(dirname "$0")
TEST_FILE="$SCRIPTPATH/data-spans.json"

# ERROR HERE

# for gRPC reflection
# cat $TEST_FILE | ghz --insecure --proto $PROTO --call opentelemetry.proto.collector.trace.v1.TraceService.Export -d @ :4317

grpcurl \
-d @ \
-plaintext \
:4317 \
opentelemetry.proto.collector.trace.v1.TraceService/Export \
< $TEST_FILE
if [ $? -eq 0 ]; then
    echo "OK: test trace sent to gRPC receiver"
else
    echo "!!! ERROR !!!"
fi