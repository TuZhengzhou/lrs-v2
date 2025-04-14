# lrs-v2

# 项目目录结构

```
lrs-v2/
├─ circoms/
│  ├─ lrs/
│  ├─ lrs_a/
│  │  ├─ lrs_a/                         # gen by lrs_a.sh. lrs_a circuit infos for ring size 2^10 ~ 2^16 
│  │  │  ├─ lrs_a_circoms/
│  │  │  ├─ generate_witness_backend.js
│  │  ├─ merkle/                        # provides merkle root for tree size 2^10 ~ 2^16
│  │  │  ├─ gen_circom.sh               # called by merkle.sh, build calls of merkel template
│  │  │  ├─ gen_input.sh                # called by merkle.sh, prepare inputs
│  │  │  ├─ merkle.sh                   # Core. To construct, evaluate merkel circoms for tree size 2^10 ~ 2^16
│  │  │  ├─ merkle.circom               # merkle template circom
│  │  ├─ lrs_a.circom                   # lrs_a template circom
│  │  ├─ lrs_a.sh                       # Core. To call lrs_a template for ring size 2^10 ~ 2^16
│  ├─ tests/
│  │  ├─ phi/                           # circuit for PK = sk G (Babyjubjub Curve)
│  │  ├─ schnorr_sign/                  # circuit for schnorr sign
│  │  ├─ schnorr_verify/                # circuit for schnorr verify
├─ logs/                # results of lrs_a & lrs_se
├─ src/
│  ├─ bin/
│  │  ├─ lrs_a.rs       # test for LRS from snarks (implemented by using cc, and set num_commit_witness=0)
│  │  ├─ lrs_se.rs      # test for LRS with simulation extractable (SE) from sma + cc + link
│  ├─ cc/               # commit-carry snarks with SE 
│  ├─ link/             # link snark with one time SE from [1] Figure.6
│  ├─ lrs/              # combine sma + cc + link
│  ├─ sma/              # set membership argument with SE
│  ├─ constants.rs
│  ├─ lib.rs
├─ tests
├─ .gitignore
├─ test_lrs_a.sh
├─ test_lrs_se.sh
├─ README.md


[1] Quasi-Adaptive NIZK for Linear Subspaces Revisited