# lrs-v2

## Project Structure

```txt
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
```

[1] Quasi-Adaptive NIZK for Linear Subspaces Revisited

## Compile && Run

### Dev Versions

- Ubuntu 20.04.6 LTS
- cargo 1.82.0 (8f40fc59f 2024-08-21)
- rustc 1.82.0 (f6e511eec 2024-10-15)
- rustup 1.27.1 (54dd3d00f 2024-04-24)
- stable-x86_64-unknown-linux-gnu (default)
- circom compiler 2.2.0 (need this only if you want to compile the circoms in /circoms)

#### To Install Rust

If you’re using Linux or macOS, open a terminal and enter the following command:

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

This will have Rust available in your system. (include cargo, rustc, rustup and a stable version)

To check whether install succeed, run:

```bash
cargo --version
```

which will output `cargo 1.86.0 (adf9b6ad1 2025-02-28)` or something like that.

#### To Install Circom

This is not forced, since we include the circuit compilation results in /circoms dirctory.

One can refer to [Circom 2 Documentation](https://docs.circom.io/getting-started/installation/) for circom installation and basic commands familarity.

### Get start

Make sure you have installed rust.

```bash
git clone 
cd lrs-v2
cargo check
cargo build --release
```

To test our LRS scheme, run:

```bash
./target/release/lrs_se
```

or

```bash
bash test_lrs_se.sh
```

the latter one redirect the output to a log file in /logs.

To run all the tests, run:

```bash
cargo test
```

### Note

If you are instersted in the circuit details and how the circuit compilation results works well with our source code, you can refer to the `/circom/lrs/run_commands.sh` file, which conatains all the step, to obtain all files we need (specifically, `name`.sym, `name`_constraints.json, `name`_js/witness.wtns.json) from the origin `name`.circom circuit.
