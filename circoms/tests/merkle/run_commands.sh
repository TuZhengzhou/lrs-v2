# 遇到报错立即停止
set -e

circom merkle.circom --r1cs --wasm --sym --json -p=bn128

snarkjs ri merkle.r1cs

snarkjs rp merkle.r1cs

# snarkjs r1cs2json merkle.r1cs

cd merkle_js

cp ../input_backend.json input.json

cp ../generate_witness_backend.js generate_witness.js

node generate_witness.js merkle.wasm input.json witness.wtns