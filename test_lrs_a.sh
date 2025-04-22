#!/bin/bash
set -e

echo "Before running this script, please ensure you have the following:"
echo "1"
echo "   cd /circoms/lrs_a"
echo "   run 'bash lrs_a.sh'(you may need to adjust var 'tree_height_low' and var 'tree_height_high' in lrs_a.sh to the desired range)\n"
echo "2"
echo "   adjust /src/constants.rs to add the macro defines that the desired range needed\n"
echo "   add 'define_input_and_circuit_names_lrs_a!(...)'"
echo "3"
echo "   adjust /src/bin/lrs_a.rs to the desired range:"
echo "   modify variable 'ioputs_names'"
echo "   modify variable 'circuit_names'"
echo "   modify variable 'low'"
echo "   modify variable 'high'"
echo "Then"
echo "    You must run this code in the root directory of current project"
echo "    You are in $(pwd)"

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

repeat=20
taskset -c 0 ./target/release/lrs_a $repeat > ./logs/lrs_a_test_$time.log 2>&1
if [ $? -ne 0 ]; then
    echo "lrs_a_test failed"
    exit 1
fi
echo "lrs_a_test succeeded"