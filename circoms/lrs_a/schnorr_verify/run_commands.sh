# 遇到报错立即停止
set -e

circom schnorr_verify.circom --r1cs --wasm --sym -p=bn128

snarkjs ri schnorr_verify.r1cs

snarkjs rp schnorr_verify.r1cs

snarkjs r1cs2json schnorr_verify.r1cs

cd schnorr_verify_js

cp ../input_backend.json input.json

cp ../generate_witness_backend.js generate_witness.js

node generate_witness.js schnorr_verify.wasm input.json witness.wtns