#!/bin/zsh

SCRIPTPATH=$(dirname "$0")
TEST_FILE="$SCRIPTPATH/data/span.json"

http POST :4318/v1/traces -v < $TEST_FILE
if [ $? -eq 0 ]; then
    echo "OK: test trace sent to HTTP receiver"
else
    echo "!!! ERROR !!!"
fi
