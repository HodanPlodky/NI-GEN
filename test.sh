#!/bin/bash

RED='\033[0;31m'
GREEN='\033[1;32m'
NC='\033[0m'

if [ ! -d examples/actual/ ]; then
    mkdir -p examples/actual
fi

function do_test() {
    ./compile -d examples/tests/${1} > examples/actual/${FILENAME}.out
    diff examples/expected/${FILENAME}.out examples/actual/${FILENAME}.out

    if [ ! "$?" -eq 0 ]; then
        printf "%s ... ${RED}fail${NC}\n" ${FILENAME}
    else
        printf "%s ... ${GREEN}ok${NC}\n" ${FILENAME}
    fi

}

for file in examples/tests/*; do
    FILENAME=$(basename -- ${file})
    if [ -f "examples/expected/${FILENAME}.out" ]; then
        do_test $FILENAME
    fi
done
