#!/bin/bash
set -e

cargo build --release

time=$(date +%Y%m%d_%H%M%S)
echo "Running lrs_a_test at $time"

if [ ! -d "./logs" ]; then
    mkdir logs
fi

if [ ! -d "./target/release" ]; then
    echo "Release directory not found. Please build the project first."
    exit 1
fi

repeat=10
./target/release/lrs_a_test $repeat > ./logs/lrs_a_test_$time.log 2>&1
if [ $? -ne 0 ]; then
    echo "lrs_a_test failed"
    exit 1
fi
echo "lrs_a_test succeeded"