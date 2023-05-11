#!/bin/zsh

SCRIPTPATH=$(dirname "$0")
TEST_FILE="$SCRIPTPATH/request.json"

cat $TEST_FILE | curl \
-X POST \
-H "Content-Type: application/json" \
--data-binary @- \
localhost:4318/v1/traces 

if [ $? -eq 0 ]; then
    echo 
    echo "OK: test trace sent to HTTP receiver"
else
    echo
    echo "!!! ERROR !!!"
fi
