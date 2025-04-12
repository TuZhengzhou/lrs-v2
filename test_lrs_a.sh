#!/bin/bash
set -e

cargo build --release

repeat=10
./target/release/lrs_a_test $repeat > lrs_a_test.log 2>&1
if [ $? -ne 0 ]; then
    echo "lrs_a_test failed"
    exit 1
fi
echo "lrs_a_test succeeded"