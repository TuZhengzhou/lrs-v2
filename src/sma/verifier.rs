use crate::cc::helpers::hash_to_field;
use crate::sma::utils::kzg_evaluate;
use crate::sma::SmaProof;

use ark_ec::{pairing::Pairing, Group};

use ark_ff::{One, Zero};
use ark_serialize::CanonicalSerialize;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;
use ark_std::time::Instant;

use std::collections::HashMap;
use crate::sma::SmaCRS;
use crate::sma::SmaComm;

#[allow(non_snake_case)]
pub fn verify_set_member_proof_opt<E: Pairing>(
    message: &str,
    sma_crs: &SmaCRS<E>,
    comm: &SmaComm<E>,
    ring: &Vec<E::ScalarField>,
    sma_proof: &SmaProof<E>,
) where
    <E::ScalarField as FromStr>::Err: Debug,
{
    let ring_size_real = ring.len() - 1; // omit the 0-th index
    let ring_size_max = sma_crs.ring_size_max; // max size of the ring

    let mut records = HashMap::new();

    let start_randomness = Instant::now();

    // Compute s and c_s_g1
    let mut common_bytes = Vec::new();
    common_bytes.extend_from_slice(message.as_bytes());
    comm.c_g1.serialize_compressed(&mut common_bytes).unwrap();
    comm.c_b_g2.serialize_compressed(&mut common_bytes).unwrap();
    let s = hash_to_field::<E>(common_bytes.clone());

    // 并行计算 s^i
    // let s_pows: Vec<E::ScalarField> = (0..=ring_size_max)
    //     .scan(E::ScalarField::one(), |state, _| {
    //         let current = *state;
    //         *state *= s;
    //         Some(current)
    //     })
    //     .collect();
    let mut s_pows: Vec<E::ScalarField> = Vec::<E::ScalarField>::with_capacity(ring_size_max + 1);
    s_pows.push(E::ScalarField::one());
    for i in 1..=ring_size_max {
        s_pows.push(s_pows[i - 1] * s);
    }

    sma_proof.c_s_g1.serialize_compressed(&mut common_bytes).unwrap();

    let t = hash_to_field::<E>(common_bytes.clone());

    let u = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        bytes.extend_from_slice(&ring_size_real.to_le_bytes());
        bytes
    });

    let t_pows: Vec<E::ScalarField> = (0..=ring_size_max)
        .scan(E::ScalarField::one(), |state, _| {
            let current = *state;
            *state *= t;
            Some(current)
        })
        .collect();

    let u_pows: Vec<E::ScalarField> = (0..=ring_size_max)
        .scan(E::ScalarField::one(), |state, _| {
            let current = *state;
            *state *= u;
            Some(current)
        })
        .collect();

    let mut delta_eq_inputs = common_bytes.clone();
    delta_eq_inputs.extend_from_slice(&1_i32.to_le_bytes());

    let delta_eq = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        bytes.extend_from_slice(&1_i32.to_le_bytes());
        bytes
    });
    let delta_b = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        s.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_o = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        bytes.extend_from_slice(&3_i32.to_le_bytes());
        bytes
    });
    let delta_d2 = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        bytes.extend_from_slice(&4_i32.to_le_bytes());
        bytes
    });
    let delta_d1 = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        bytes.extend_from_slice(&5_i32.to_le_bytes());
        bytes
    });
    let delta_phi = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        bytes.extend_from_slice(&6_i32.to_le_bytes());
        bytes
    });

    let duration_randomness = start_randomness.elapsed();
    records.insert(
        "Gen randomness",
        format!(
            "{}.{:09} seconds",
            duration_randomness.as_secs(),
            duration_randomness.subsec_nanos()
        ),
    );

    let start_verify_1_poly = Instant::now();
        // Computation outsourced to the prover
    let mut poly_1 = vec![E::ScalarField::zero(); ring_size_max + 1]; // poly_e12_left_up_g1_outsource
    let mut poly_2 = vec![E::ScalarField::zero(); ring_size_max + 1]; // poly_e12_left_down1_g1_outsource
    let mut poly_3 = vec![E::ScalarField::zero(); ring_size_max + 1]; // poly_e12_left_down2_g2_outsource

    // Compute the polynomial for the left-up part of the equation
    for i in 1..=ring_size_max {
        poly_1[ring_size_max + 1 - i] += (delta_eq * t_pows[i] - delta_b) * s_pows[i];
    }
    for i in 1..=ring_size_real {
        poly_1[ring_size_max + 1 - i] += delta_o + delta_phi * ring[i];
    }

    // Compute the polynomial for the left-down part of the equation
    for i in 2..=ring_size_max {
        poly_2[ring_size_max + 1 - i] = -delta_d1 * u_pows[i];
    }
    for i in 1..=ring_size_max {
        poly_3[i] = delta_eq * t_pows[i];
    }

    let duration_verify_1_poly = start_verify_1_poly.elapsed();
    records.insert(
        "Verify 1 poly",
        format!(
            "{}.{:09} seconds",
            duration_verify_1_poly.as_secs(),
            duration_verify_1_poly.subsec_nanos()
        ),
    );

    // Generate random scalars for outsourced computation
    let start_randomness_outsource_proof = Instant::now();
    let z = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        sma_proof.pi.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_h = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        sma_proof.c_h_g1.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_u = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        sma_proof.c_u_g2.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_t = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        sma_proof.c_t_g2.serialize_compressed(&mut bytes).unwrap();
        bytes
    });

    let duration_randomness_outsource_proof = start_randomness_outsource_proof.elapsed();
    records.insert(
        "Gen randomness outsource proof",
        format!(
            "{}.{:09} seconds",
            duration_randomness_outsource_proof.as_secs(),
            duration_randomness_outsource_proof.subsec_nanos()
        ),
    );

    let start_verify_2_eval = Instant::now();
    // Evaluate the polynomials at r_1, r_2, r_3 respectively
    let eval_1 = kzg_evaluate::<E>(&poly_1, z);
    let eval_2 = kzg_evaluate::<E>(&poly_2, z);
    let eval_3 = kzg_evaluate::<E>(&poly_3, z);
    let duration_verify_2_eval = start_verify_2_eval.elapsed();
    records.insert(
        "Verify 2 eval",
        format!(
            "{}.{:09} seconds",
            duration_verify_2_eval.as_secs(),
            duration_verify_2_eval.subsec_nanos()
        ),
    );

    let start_verify_3_outsource_verify = Instant::now();
    // Perform pairing checks
    assert_eq!(
        E::pairing(E::G1::from(sma_proof.c_h_g1) - E::G1::generator() * eval_1, E::G2::generator() * delta_h) + 
            E::pairing(E::G1::generator() * delta_u, E::G2::from(sma_proof.c_u_g2) - E::G2::generator() * eval_2) +
            E::pairing(E::G1::generator() * delta_t, E::G2::from(sma_proof.c_t_g2) - E::G2::generator() * eval_3),

        E::pairing(sma_proof.pi_kzg, E::G2::from(sma_crs.crs_g2s[1]) - E::G2::generator() * z)
    );

    let duration_verify_3_outsource_verify = start_verify_3_outsource_verify.elapsed();
    records.insert(
        "Verify 3 outsource verify",
        format!(
            "{}.{:09} seconds",
            duration_verify_3_outsource_verify.as_secs(),
            duration_verify_3_outsource_verify.subsec_nanos()
        ),
    );

    let start_verify_4_pi = Instant::now();
    let e12_left_up_g1 = sma_proof.c_s_g1 * delta_b + sma_proof.c_h_g1;
    let e12_left_down1_g2 = E::G2::from(sma_crs.crs_g2s[ring_size_max]) * delta_phi + sma_proof.c_u_g2 - E::G2::from(sma_crs.crs_g2s[ring_size_max-1]) * delta_d2;
    let e12_left_down2_g2 = sma_proof.c_t_g2;
    assert!(
        E::pairing(e12_left_up_g1, sma_proof.c_b_g2) +
            - (E::pairing(comm.c_g1, e12_left_down1_g2)
                + E::pairing(sma_proof.c_s_g1, e12_left_down2_g2)
                + E::pairing(sma_crs.crs_g1s[ring_size_max] * delta_o, sma_crs.crs_g2s[1]))
            == E::pairing(sma_proof.pi, E::G2::generator())
    );
    let duration_verify_4_pi = start_verify_4_pi.elapsed();
    records.insert(
        "Verify 4 pi",
        format!(
            "{}.{:09} seconds",
            duration_verify_4_pi.as_secs(),
            duration_verify_4_pi.subsec_nanos()
        ),
    );

    let duration_opt = duration_verify_1_poly
        + duration_verify_2_eval
        + duration_verify_3_outsource_verify
        + duration_verify_4_pi;
    records.insert(
        "Opt",
        format!(
            "{}.{:09} seconds",
            duration_opt.as_secs(),
            duration_opt.subsec_nanos()
        ),
    );

    // println!("=============================================VERIFY============================================================");
    // for (key, value) in records.iter() {
    //     println!("{:<30}: {}", key, value);
    // }
}
