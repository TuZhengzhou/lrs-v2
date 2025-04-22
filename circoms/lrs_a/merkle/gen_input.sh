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

dirname="input_backends"

if ! [ -d "$dirname" ]; then
  mkdir $dirname
  echo "Directory $dirname created."
else
  echo "Directory $dirname already exists."
fi

for n in $(seq $tree_height_low $tree_height_high); do
  # 生成pathElements数组
  path_elements=()
  for i in $(seq 1 $n); do
    path_elements+=("\"$i\"")
  done
  elements_str=$(IFS=,; echo "${path_elements[*]}")

  # 生成pathIndices数组（交替1和0）
  path_indices=()
  for i in $(seq 1 $n); do
    if (( i % 2 )); then
      path_indices+=("\"1\"")
    else
      path_indices+=("\"0\"")
    fi
  done
  indices_str=$(IFS=,; echo "${path_indices[*]}")

  # 生成JSON文件
  cat > "./$dirname/input_backend_${n}.json" <<EOF
{
    "leaf": "16522763539274936672550010747276183294750403510407374019801689147715102349978",
    "pathElements": [${elements_str}],
    "pathIndices": [${indices_str}]
}
EOF

done