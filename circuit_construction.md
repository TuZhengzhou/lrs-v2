# Building the Circuit and Generating Proofs

This section uses [Circom](https://github.com/iden3/circom) to compile the circuit and generate proofs, following steps similar to those in the [snarkjs tutorial](https://docs.circom.io/getting-started/writing-circuits/). The goal is to obtain the `circuit_constraints.json`, `circuit.sym`, and `witness.wtns.json` files to build the constraint system supported by `cc` (commit-and-carry version of Groth16).

---

## Example Circuit

As an example, consider the `circuit.circom` file:

```circom
pragma circom 2.1.4;

template Multiplier() {
   signal input a;
   signal input b;
   signal output c;
   c <== a*b;
}

component main = Multiplier();
```

---

## Steps

All commands are integrated into a `./circoms/lrs/run_command.sh` file, which you can execute by running `bash run_command.sh`. Below is an introduction to what each step in the `run_command.sh` file does:

---

### 1. Compile the circuit

```bash
circom circuit.circom --r1cs --wasm --sym --json -p=bn128
```

- `--r1cs`: Generates an `.r1cs` binary file describing the circuit in the R1CS constraint system  
- `--wasm`: Produces a folder named `circuit_js` (matching the circuit name), containing files for witness generation such as `circuit.wasm`, `generate_witness.js`, and `witness_calculator.js`  
- `--sym`: Generates a `.sym` symbol file that labels the variables used in the circuit  
- `--json`: Outputs the circuit constraints `circuit_constraints.json` in JSON format  
- `-p=bn128`: Specifies the finite field, choosing the scalar field of `bn128`

---

### 2. View circuit information

```bash
snarkjs ri circuit.r1cs
snarkjs rp circuit.r1cs
```

---

## Generating the Proof

`snarkjs` uses a JSON file as input. We have prepared an example input file named `input_backend.json` in the directory where `run_command.sh` is located. We remark that don't change the contents of any `input_backend.json` file, as it is obtained by other related circuits.

---

### 4. Generate the witness file

Inside the `circuit_js` directory, run the following command to generate the `witness.wtns` file:

```bash
node generate_witness.js circuit.wasm input.json witness.wtns
```

Since `witness.wtns` is a non-human-readable binary file, we modified `generate_witness.js` to also output a `witness.wtns.json` file.

> **Note**:  
>
> - By default, `generate_witness.js` is automatically created after running `circom circuit.circom --wasm`.  
> - We renamed the modified file to `generate_witness_backend.js` and replaced the original file with it:

```bash
cp ../generate_witness_backend.js generate_witness.js
node generate_witness.js circuit.wasm input.json witness.wtns
```

---

## File Descriptions

- **circuit_constraints.json**:  
  A file describing the constraint system (variables and their coefficients).

- **circuit.sym**:  
  The symbol table. Column 2 denotes the variable index, while column 4 indicates the variable name.

- **witness.wtns.json**:  
  A JSON file containing a mapping from variable indices to their respective values.

By combining these three files, you can construct the constraint system supported by `cc`.
