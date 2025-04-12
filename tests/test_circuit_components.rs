extern crate lrs_v2;

use lrs_v2::lrs::lrs_circ::LRSCirc;
use lrs_v2::lrs::structures::CircDescriptor;
use ark_ec::pairing::Pairing;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;
use lrs_v2::cc::utils::cc_prove_and_verify;


/// Prove and verify phi
pub fn cc_prove_and_verify_phi<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let num_pub_io = 1usize;
    let num_commit_witness = 1usize;
    let ioputs_name = vec!["main.phi", "main.sk"];
    let path_prefix = "./circoms/tests/phi/";
    let circuit_name = "phi";

    let circ_desc = CircDescriptor {
        num_pub_io,
        num_commit_witness,
        ioputs_name: ioputs_name.iter().map(|s| s.to_string()).collect(),
        path_prefix: path_prefix.to_string(),
        circuit_name: circuit_name.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}


/// Information of Merkle circuit for the CircDescriptor
pub const NUM_PUB_IO_MERKLE: usize = 2;
pub const NUM_COMMIT_WITNESS_MERKLE: usize = 2;
pub const IOPUTS_NAME_MERKLE: [&str; 32] = [
    "main.root",
    "main.leaf",
    "main.pathElements[0]",
    "main.pathElements[1]",
    "main.pathElements[2]",
    "main.pathElements[3]",
    "main.pathElements[4]",
    "main.pathElements[5]",
    "main.pathElements[6]",
    "main.pathElements[7]",
    "main.pathElements[8]",
    "main.pathElements[9]",
    "main.pathElements[10]",
    "main.pathElements[11]",
    "main.pathElements[12]",
    "main.pathElements[13]",
    "main.pathElements[14]",
    "main.pathIndices[0]",
    "main.pathIndices[1]",
    "main.pathIndices[2]",
    "main.pathIndices[3]",
    "main.pathIndices[4]",
    "main.pathIndices[5]",
    "main.pathIndices[6]",
    "main.pathIndices[7]",
    "main.pathIndices[8]",
    "main.pathIndices[9]",
    "main.pathIndices[10]",
    "main.pathIndices[11]",
    "main.pathIndices[12]",
    "main.pathIndices[13]",
    "main.pathIndices[14]",
];
pub const PATH_PREFIX_MERKLE: &str = "./circoms/lrs_a/merkle/merkle_circoms/";
pub const CIRCUIT_NAME_MERKLE: &str = "merkle_15";

/// Prove and verify Merkle tree
pub fn cc_prove_and_verify_merkle<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_MERKLE,
        num_commit_witness: NUM_COMMIT_WITNESS_MERKLE,
        ioputs_name: IOPUTS_NAME_MERKLE.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_MERKLE.to_string(),
        circuit_name: CIRCUIT_NAME_MERKLE.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}


/// Information of SchnorrSign circuit for the CircDescriptor
pub const NUM_PUB_IO_SCHNORR_SIGN: usize = 4;
pub const NUM_COMMIT_WITNESS_SCHNORR_SIGN: usize = 1;
pub const IOPUTS_NAME_SCHNORR_SIGN: [&str; 8] = [
    "main.msg",
    "main.R[0]",
    "main.R[1]",
    "main.s",
    "main.sk",
    "main.pk[0]",
    "main.pk[1]",
    "main.k",
];
pub const PATH_PREFIX_SCHNORR_SIGN: &str = "./circoms/tests/schnorr_sign/";
pub const CIRCUIT_NAME_SCHNORR_SIGN: &str = "schnorr_sign";

/// Prove and verify Schnorr signature generation
pub fn cc_prove_and_verify_schnorr_sign<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_COMMIT_WITNESS_SCHNORR_SIGN,
        num_commit_witness: NUM_COMMIT_WITNESS_SCHNORR_SIGN,
        ioputs_name: IOPUTS_NAME_SCHNORR_SIGN
            .iter()
            .map(|s| s.to_string())
            .collect(),
        path_prefix: PATH_PREFIX_SCHNORR_SIGN.to_string(),
        circuit_name: CIRCUIT_NAME_SCHNORR_SIGN.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}


/// Information of SchnorrVerify circuit for the CircDescriptor
pub const NUM_PUB_IO_SCHNORR_VERIFY: usize = 6;
pub const NUM_COMMIT_WITNESS_SCHNORR_VERIFY: usize = 0;
pub const IOPUTS_NAME_SCHNORR_VERIFY: [&str; 7] = [
    "main.msg",
    "main.R[0]",
    "main.R[1]",
    "main.s",
    "main.pk[0]",
    "main.pk[1]",
    "main.sk",
];
pub const PATH_PREFIX_SCHNORR_VERIFY: &str = "./circoms/tests/schnorr_verify/";
pub const CIRCUIT_NAME_SCHNORR_VERIFY: &str = "schnorr_verify";

/// Prove and verify Schnorr signature verification
pub fn cc_prove_and_verify_schnorr_verify<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_SCHNORR_VERIFY,
        num_commit_witness: NUM_COMMIT_WITNESS_SCHNORR_VERIFY,
        ioputs_name: IOPUTS_NAME_SCHNORR_VERIFY
            .iter()
            .map(|s| s.to_string())
            .collect(),
        path_prefix: PATH_PREFIX_SCHNORR_VERIFY.to_string(),
        circuit_name: CIRCUIT_NAME_SCHNORR_VERIFY.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}


/// Information of LRS_A circuit for the CircDescriptor
pub const NUM_PUB_IO_LRS_A: usize = 7;
pub const NUM_COMMIT_WITNESS_LRS_A: usize = 0;
pub const IOPUTS_NAME_LRS_A: [&str; 41] = [
    "main.root",
    "main.sc",
    "main.L",
    "main.msg",
    "main.R[0]",
    "main.R[1]",
    "main.s",
    "main.sk",
    "main.pk[0]",
    "main.pk[1]",
    "main.phi",
    "main.pathElements[0]",
    "main.pathElements[1]",
    "main.pathElements[2]",
    "main.pathElements[3]",
    "main.pathElements[4]",
    "main.pathElements[5]",
    "main.pathElements[6]",
    "main.pathElements[7]",
    "main.pathElements[8]",
    "main.pathElements[9]",
    "main.pathElements[10]",
    "main.pathElements[11]",
    "main.pathElements[12]",
    "main.pathElements[13]",
    "main.pathElements[14]",
    "main.pathIndices[0]",
    "main.pathIndices[1]",
    "main.pathIndices[2]",
    "main.pathIndices[3]",
    "main.pathIndices[4]",
    "main.pathIndices[5]",
    "main.pathIndices[6]",
    "main.pathIndices[7]",
    "main.pathIndices[8]",
    "main.pathIndices[9]",
    "main.pathIndices[10]",
    "main.pathIndices[11]",
    "main.pathIndices[12]",
    "main.pathIndices[13]",
    "main.pathIndices[14]",
];
pub const PATH_PREFIX_LRS_A: &str = "./circoms/lrs_a/lrs_a/lrs_a_circoms/";
pub const CIRCUIT_NAME_LRS_A: &str = "lrs_a_15";

/// Prove and verify LRS_A
pub fn cc_prove_and_verify_lrs_a<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_LRS_A,
        num_commit_witness: NUM_COMMIT_WITNESS_LRS_A,
        ioputs_name: IOPUTS_NAME_LRS_A.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_LRS_A.to_string(),
        circuit_name: CIRCUIT_NAME_LRS_A.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}

#[test]
fn test_cc_prove_and_verify_phi() {
    cc_prove_and_verify_phi::<ark_bn254::Bn254>(1);
}

#[test]
fn test_cc_prove_and_verify_merkle() {
    cc_prove_and_verify_merkle::<ark_bn254::Bn254>(1);
}

#[test]
fn test_cc_prove_and_verify_schnorr_sign() {
    cc_prove_and_verify_schnorr_sign::<ark_bn254::Bn254>(1);
}

#[test]
fn test_cc_prove_and_verify_schnorr_verify() {
    cc_prove_and_verify_schnorr_verify::<ark_bn254::Bn254>(1);
}

#[test]
fn test_cc_prove_and_verify_lrs_a() {
    cc_prove_and_verify_lrs_a::<ark_bn254::Bn254>(1);
}

