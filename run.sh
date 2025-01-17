#!/bin/sh

if [ $# -eq 0 ]; then
    echo "Need to supply day number as parameter"
    exit 1
fi

cargo run --release --bin day$1 -- input/day$1
