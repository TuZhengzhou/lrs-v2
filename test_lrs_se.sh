# !/bin/bash
set -e

cargo build --release

repeat=10
./target/release/lrs_se $repeat > lrs_se_test.log 2>&1
if [ $? -ne 0 ]; then
    echo "lrs_se failed"
    exit 1
fi
echo "lrs_se succeeded"