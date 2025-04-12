# !/bin/bash
set -e

cargo build --release

time=$(date +%Y%m%d_%H%M%S)
echo "Running lrs_se at $time"

if [ ! -d "./logs" ]; then
    mkdir logs
fi

if [ ! -d "./target/release" ]; then
    echo "Release directory not found. Please build the project first."
    exit 1
fi

repeat=100
./target/release/lrs_se $repeat > ./logs/lrs_se_test_$time.log 2>&1
if [ $? -ne 0 ]; then
    echo "lrs_se failed"
    exit 1
fi
echo "lrs_se succeeded"