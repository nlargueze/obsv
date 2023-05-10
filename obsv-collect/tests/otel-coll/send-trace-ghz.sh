#!/bin/zsh

SCRIPT_PATH=$(dirname "$0")
DATA="./tests/data/span.json"
PROTO="./collector/trace/v1/trace_service.proto"
echo $PROTO

ghz --insecure --proto $PROTO --call TraceService.Export -d '{"name": "nick"}' 0.0.0.0:4317 

if [ $? -eq 0 ]; then
    echo "OK: test trace sent to gRPC receiver"
else
    echo "!!! ERROR !!!"
fi