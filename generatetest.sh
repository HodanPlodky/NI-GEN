#!/bin/bash

FILENAME=$(basename -- ${1})

if [ ! -d examples/expected/ ]; then
    mkdir -p examples/expected
fi

./compile -d ${1} > examples/expected/${FILENAME}.out
