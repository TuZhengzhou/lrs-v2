# 遇到报错立即停止
set -e

circom phi.circom --r1cs --wasm --sym -p=bn128

snarkjs ri phi.r1cs

snarkjs rp phi.r1cs

snarkjs r1cs2json phi.r1cs

cd phi_js

cp ../input_backend.json input.json

cp ../generate_witness_backend.js generate_witness.js

node generate_witness.js phi.wasm input.json witness.wtns