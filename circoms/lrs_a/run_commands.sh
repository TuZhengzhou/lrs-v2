set -e

circom lrs_a.circom --r1cs --wasm --sym --json -p=bn128

snarkjs ri lrs_a.r1cs

snarkjs rp lrs_a.r1cs

# snarkjs r1cs2json lrs_a.r1cs

cd lrs_a_js

cp ../input_backend.json input.json

cp ../generate_witness_backend.js generate_witness.js

node generate_witness.js lrs_a.wasm input.json witness.wtns