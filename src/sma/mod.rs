pub mod structures;
pub mod generator;
pub mod prover;
pub mod verifier;
pub mod utils;

pub use structures::*;
pub use generator::*;
pub use prover::*;
pub use verifier::*;
pub use utils::*;

#[test]
fn test_set_member_proof_opt() {
    use ark_bn254::Bn254;
    use ark_std::collections::HashMap;
    use ark_std::time::Instant;

    let rng = &mut ark_std::test_rng();
    let ring_size_max = 1 << 6;
    let ring_size_real = 1 << 6;
    let mut records = HashMap::new();

    println!("=============================================PARAMETERS============================================================");
    println!("{:<30}: {}", "Max Rings Size", ring_size_max.to_string());
    println!("{:<30}: {}", "Ring Size", ring_size_real.to_string());

    let security_par = "128".to_string();
    // let _alpha = crs_key_gen::<_, Bn254>(security_par, ring_size_max, rng, &mut crs_g1s, &mut crs_g2s);
    let sma_crs = crs_key_gen(security_par, ring_size_max, rng);

    let ring = ring_gen::<_, Bn254>(ring_size_real, 1, rng);
    let sma_comm = commit::<_, Bn254>(&ring, &sma_crs, 1, rng);
    
    let msg = "test message";
    let sma_proof = set_member_proof_opt::<_, Bn254>(
        msg,
        &sma_crs,
        &sma_comm,
        &ring,
        1,
        rng,
    );

    let t1 = Instant::now();
    verify_set_member_proof_opt::< Bn254>(
        msg,
        &sma_crs,
        &sma_comm,
        &ring,
        &sma_proof,
    );
    let t2 = Instant::now();
    records.insert(
        "Verify proof (More than all)",
        format!(
            "{}.{:09} seconds",
            t2.duration_since(t1).as_secs(),
            t2.duration_since(t1).subsec_nanos()
        ),
    );

    println!("=============================================TIME============================================================");
    for (key, value) in records.iter() {
        println!("{:<30}: {}", key, value);
    }
}
