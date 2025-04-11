set -e

circom lrs.circom --r1cs --wasm --sym --json -p=bn128

snarkjs ri lrs.r1cs

snarkjs rp lrs.r1cs

# snarkjs r1cs2json lrs.r1cs

cd lrs_js

echo '{"sk":"20001027", "sc":"20000928", "phi": "15203260804472416887611568188415855230460508361971914869329234349513402114304", "L": "17804474404831372771318062141310527328296588970581814948270177988694117180422"}' > input.json

cp ../generate_witness_backend.js generate_witness.js

node generate_witness.js lrs.wasm input.json witness.wtns