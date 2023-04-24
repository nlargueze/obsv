#!/bin/zsh

SCRIPTPATH=$(dirname "$0")
TEST_FILE="$SCRIPTPATH/span.json"

# for gRPC reflection
# grpcurl -plaintext list

grpcurl -d @ -insecure :4317 TraceService.Export < $TEST_FILE

if [ $? -eq 0 ]; then
    echo "OK: test trace sent to gRPC receiver"
else
    echo "!!! ERROR !!!"
fi