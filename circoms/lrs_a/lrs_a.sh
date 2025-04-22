set -e

tree_height_low=3
tree_height_high=20

cd ./merkle
bash merkle.sh $tree_height_low $tree_height_high
cd ..

echo "You must run this code in the /circoms/lrs_a directory of current project"
echo "You are in $(pwd)"

cd ./lrs_a
dirname="lrs_a_circoms"

if ! [ -d "$dirname" ]; then
    mkdir $dirname
    echo "Directory $dirname created."  
fi

cd $dirname

for i in $(seq $tree_height_low $tree_height_high); do

    # 生成 Circom 文件
    template_name="lrs_a"
    name=$template_name\_$i
    echo $name

    cat > $name.circom <<EOF
    pragma circom 2.2.0;

    include "../../$template_name.circom";

    component main = SNARKCircuitA($i);
EOF

    # 生成pathElements数组
    path_elements=()
    for j in $(seq 1 $i); do
        path_elements+=("\"$j\"")
    done
    elements_str=$(IFS=,; echo "${path_elements[*]}")

    # 生成pathIndices数组（交替1和0）
    path_indices=()
    for j in $(seq 1 $i); do
        if (( j % 2 )); then
        path_indices+=("\"1\"")
        else
        path_indices+=("\"0\"")
        fi
    done
    indices_str=$(IFS=,; echo "${path_indices[*]}")

    # 读取 root 值
    root_file=../../merkle/merkle_circoms/merkle_$i\_js/witness.wtns.json
    echo "Current directory: $(pwd)"
    echo "Root file: $root_file"
    if [ ! -f "$root_file" ]; then
        echo "File $root_file not found!"
        exit 1
    fi
    # 使用 jq 读取 root 值
    root_value=$(jq -r '[."1"] | .[]' $root_file)
    echo "Root value: $root_value"

    # 生成Input文件
    cat > "input_backend_${i}.json" <<EOF
{
    "sk": "20001027",
    "sc": "20000928",
    "phi": "16522763539274936672550010747276183294750403510407374019801689147715102349978",
    "L": "17804474404831372771318062141310527328296588970581814948270177988694117180422",
    "msg": "12345",
    "pathElements": [${elements_str}],
    "pathIndices": [${indices_str}],
    "root": "$root_value",
    "R": [
        "10918642783053875021381551257981477660228172875824202511341991363438302410352",
        "7976072897088337598522169879450513734342614653470297207233748225578910562791"
    ],
    "s": "10906470673929214899118628593245015559688844497719200689591735429009794817349"
}
EOF

    echo $name
    circom $name.circom --r1cs --wasm --sym --json -p=bn128
    snarkjs ri $name.r1cs

    # snarkjs rp $name.r1cs
    # snarkjs r1cs2json merkle.r1cs

    cd $name\_js
    echo $name\_js

    cp ../input_backend_$i.json input.json
    cp ../../generate_witness_backend.js generate_witness.js
    node generate_witness.js $name.wasm input.json witness.wtns

    cd ..

done
