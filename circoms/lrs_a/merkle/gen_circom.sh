#!/bin/bash
set -e

tree_height_low=$1
tree_height_high=$2
if [ -z "$tree_height_low" ] || [ -z "$tree_height_high" ]; then
    echo "Usage: $0 <tree_height_low> <tree_height_high>"
    tree_height_low=3
    tree_height_high=16
    echo "Defaulting to tree height range: $tree_height_low to $tree_height_high"
fi

dirname="merkle_circoms"

if ! [ -d "$dirname" ]; then
    mkdir $dirname
    echo "Directory $dirname created."  
fi


for i in $(seq $tree_height_low $tree_height_high); do
name=merkle_$i
echo "merkle_$i"

# echo -e "pragma circom 2.2.0; \n\ninclude \"../include/merkle.circom\";  \n\ncomponent main = MerkleTreeChecker($i);" > $name.circom

    # 生成JSON文件
    cat > ./$dirname/$name.circom <<EOF
    pragma circom 2.2.0;

    include "../merkle.circom";

    component main = MerkleTreeChecker($i);
EOF

done