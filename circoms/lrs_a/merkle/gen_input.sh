#!/bin/bash

set -e

dirname="input_backends"

if ! [ -d "$dirname" ]; then
  mkdir $dirname
  echo "Directory $dirname created."
else
  echo "Directory $dirname already exists."
fi

for n in {10..16}; do
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