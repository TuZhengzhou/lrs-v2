# 遇到报错立即停止
set -e

cirdir="merkle_circoms"
inputdir="input_backends"

if ! [ -d "$cirdir" ]; then
    echo "Directory $cirdir created."
    bash gen_circom.sh
fi
if ! [ -d "$inputdir" ]; then
    echo "Directory $inputdir created."
    bash gen_input.sh
fi

cd $cirdir
for i in $(seq 10 16); do
    
    name=merkle_$i
    echo $name

    circom $name.circom --r1cs --wasm --sym -p=bn128
    snarkjs ri $name.r1cs

    # snarkjs rp $name.r1cs
    # snarkjs r1cs2json merkle.r1cs

    cd $name\_js
    echo $name\_js

    cp ../../$inputdir/input_backend_$i.json input.json
    cp ../../generate_witness_backend.js generate_witness.js
    node generate_witness.js $name.wasm input.json witness.wtns

    cd ..
done

