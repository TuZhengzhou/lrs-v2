# 遇到报错立即停止
set -e

tree_height_low=$1
tree_height_high=$2
if [ -z "$tree_height_low" ] || [ -z "$tree_height_high" ]; then
    echo "Usage: $0 <tree_height_low> <tree_height_high>"
    tree_height_low=3
    tree_height_high=16
    echo "Defaulting to tree height range: $tree_height_low to $tree_height_high"
fi

cirdir="merkle_circoms"
inputdir="input_backends"

# if ! [ -d "$cirdir" ]; then
#     echo "Directory $cirdir created."
#     bash gen_circom.sh
# fi
# if ! [ -d "$inputdir" ]; then
#     echo "Directory $inputdir created."
#     bash gen_input.sh
# fi

bash gen_circom.sh $tree_height_low $tree_height_high
bash gen_input.sh $tree_height_low $tree_height_high
# echo "You must run this code in the /circoms/lrs_a/merkle directory of current project"

cd $cirdir
for i in $(seq $tree_height_low $tree_height_high); do
    
    name=merkle_$i
    echo $name

    circom $name.circom --r1cs --wasm --sym --json -p=bn128
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

