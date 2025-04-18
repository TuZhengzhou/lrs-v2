set -e

echo "You must run this code in the /circoms/lrs directory of current project"
echo "You are in $(pwd)"

circom lrs.circom --r1cs --wasm --sym --json -p=bn128

snarkjs ri lrs.r1cs

snarkjs rp lrs.r1cs

# snarkjs r1cs2json lrs.r1cs

cd lrs_js

cp ../input_backend.json input.json

cp ../generate_witness_backend.js generate_witness.js

node generate_witness.js lrs.wasm input.json witness.wtns