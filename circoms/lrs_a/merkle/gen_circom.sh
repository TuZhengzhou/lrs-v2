#!/bin/bash
set -e

dirname="merkle_circoms"

if ! [ -d "$dirname" ]; then
    mkdir $dirname
    echo "Directory $dirname created."  
fi


for i in $(seq 10 16); do
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