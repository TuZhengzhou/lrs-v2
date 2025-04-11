use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_std::time::Instant;
use lrs_v2::constants::*;
use lrs_v2::lrs::setup;
use lrs_v2::lrs::sign;
use lrs_v2::lrs::verify;
use lrs_v2::lrs::CircDescriptor;
use lrs_v2::sma::ring_gen;

type F = <Bn254 as Pairing>::ScalarField;
type G1 = <Bn254 as Pairing>::G1;

fn main() {
    let security_par = "128".to_string();
    let msg = "test message";
    let ring_size_max = 1 << 10;

    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_LRS_SE,
        num_commit_witness: NUM_COMMIT_WITNESS_LRS_SE,
        ioputs_name: IOPUTS_NAME_LRS_SE.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_LRS_SE.to_string(),
        circuit_name: CIRCUIT_NAME_LRS_SE.to_string(),
    };

    let (lrs_pvkey, circuit) =
        setup::setup::<ark_bn254::Bn254>(security_par, ring_size_max, &circ_desc);

    let signer_idx = 1usize;
    let rng = &mut ark_std::test_rng();

    let mut ring = ring_gen::<_, ark_bn254::Bn254>(ring_size_max, signer_idx, rng);
    ring[signer_idx] = circuit.commit_witness[0];

    println!("Public inputs:");
    for i in 0..circ_desc.num_pub_io {
        let value: F = F::from_bigint(circuit.instance[i].into_bigint()).unwrap();
        println!("{}: {}", circ_desc.ioputs_name[i], value);
    }

    println!("Commit witness:");
    for i in 0..circ_desc.num_commit_witness {
        let value: F = F::from_bigint(circuit.commit_witness[i].into_bigint()).unwrap();
        println!(
            "{}: {}",
            circ_desc.ioputs_name[circ_desc.num_pub_io + i],
            value
        );
    }

    let signature = sign::sign::<ark_bn254::Bn254>(&lrs_pvkey, &circuit, &ring, msg);

    let start_verify = Instant::now();
    let result = verify::verify::<ark_bn254::Bn254>(&lrs_pvkey, &ring, msg, &signature);
    let verify_time = start_verify.elapsed();
    println!("Signature verification time: {:?}", verify_time);

    assert!(result, "Signature verification failed");

    use ark_ec::AffineRepr;
    use ark_ec::CurveGroup;
    use ark_ec::Group;
    println!(
        "BN254 G1 generator.x: {:?}",
        G1::generator().into_affine().x()
    );
    println!(
        "BN254 G1 generator.y: {:?}",
        G1::generator().into_affine().y()
    );
}
