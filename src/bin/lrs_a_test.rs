use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_std::time::Instant;
use ark_std::One;
use lrs_v2::cc;
use lrs_v2::cc::helpers::multi_mimc7;
use lrs_v2::cc::{verify_proof, CcPVKey};
use lrs_v2::constants::*;
use lrs_v2::lrs::lrs_circ::LRSCirc;
use lrs_v2::lrs::structures::CircDescriptor;

fn main() {
    type E = ark_bn254::Bn254;
    type F = <E as Pairing>::ScalarField;

    // 将println 输出重定向到文件
    // ./target/release/lrs_a_test > ./target/release/lrs_a_test.log

    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_LRS_A,
        num_commit_witness: NUM_COMMIT_WITNESS_LRS_A,
        ioputs_name: IOPUTS_NAME_LRS_A.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_LRS_A.to_string(),
        circuit_name: CIRCUIT_NAME_LRS_A.to_string(),
    };

    let circ = LRSCirc::<F>::construct(&circ_desc).unwrap();

    let mut setup_times = Vec::new();
    let mut proof_times = Vec::new();
    let mut verify_times = Vec::new();

    let n_iters = 100;
    for repeat in 0..n_iters {
        let rng = &mut ark_std::test_rng();

        let start = Instant::now();
        let crs_cc: CcPVKey<E> =
            cc::generate_random_parameters(circ.clone(), circ_desc.num_commit_witness, rng)
                .unwrap();
        let setup_time = start.elapsed();
        setup_times.push(setup_time);
        println!("Repeat {:?}: LRS_A Setup time: {:?}", repeat, setup_time);

        let start = Instant::now();
        let (cc_proof, _, _) = cc::create_random_proof(circ.clone(), &crs_cc.pk, rng).unwrap();
        let proof_time = start.elapsed();
        proof_times.push(proof_time);
        println!("Repeat {:?}: LRS_A Proof time: {:?}", repeat, proof_time);

        let instance = circ
            .instance
            .iter()
            .map(|v| v.into_bigint())
            .collect::<Vec<<F as ark_ff::PrimeField>::BigInt>>();
        let start = Instant::now();
        let result = verify_proof(&crs_cc.vk, &cc_proof, &instance);
        // The verifier also need to recompute the Merkle root, which takes 2^15 - 1 times MIMC hash
        for _ in 0..(1 << 15) - 1 {
            let _ = multi_mimc7::<E>(&vec![F::one(), F::one()], 2);
        }
        let verify_time = start.elapsed();
        verify_times.push(verify_time);
        println!("Repeat {:?}: LRS_A Verify time: {:?}", repeat, verify_time);

        assert!(result.is_ok());
    }
    let avg_setup_time = setup_times.iter().sum::<std::time::Duration>() / n_iters as u32;
    let avg_proof_time = proof_times.iter().sum::<std::time::Duration>() / n_iters as u32;
    let avg_verify_time = verify_times.iter().sum::<std::time::Duration>() / n_iters as u32;
    println!("Average Setup time: {:?}", avg_setup_time);
    println!("Average Proof time: {:?}", avg_proof_time);
    println!("Average Verify time: {:?}", avg_verify_time);
}
