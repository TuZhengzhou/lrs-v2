use crate::cc::helpers::hash_to_field;
use crate::sma::utils::kzg_evaluate;
use crate::sma::{SmaCRS, SmaProof};
use ark_ec::VariableBaseMSM;
use ark_ec::{pairing::Pairing, Group};
use ark_ff::PrimeField;
use ark_ff::{One, Zero};
use ark_serialize::CanonicalSerialize;
use ark_std::cfg_iter;
use ark_std::fmt::Debug;
use ark_std::rand::RngCore;
use ark_std::str::FromStr;
use ark_std::time::Instant;
use ark_std::UniformRand;
use rayon::iter::IntoParallelRefIterator;
use std::{collections::HashMap, ops::Mul};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use super::SmaComm;

#[allow(non_snake_case)]
pub fn set_member_proof_opt<R: RngCore, E: Pairing>(
    message: &str,
    sma_crs: &SmaCRS<E>,
    comm: &SmaComm<E>,
    ring: &Vec<E::ScalarField>,
    signer_index: usize,
    rng: &mut R,
) -> SmaProof<E>
where
    <E::ScalarField as FromStr>::Err: Debug,
{
    let ring_size_real = ring.len() - 1; // omit the 0-th index
    let ring_size_max = sma_crs.ring_size_max; // max size of the ring

    let mut records = HashMap::new();

    let start_randomness = Instant::now();
    let r_s = E::ScalarField::rand(rng);

    // Compute s and c_s_g1
    let mut common_bytes = Vec::new();
    common_bytes.extend_from_slice(message.as_bytes());
    comm.c_g1.serialize_compressed(&mut common_bytes).unwrap();
    comm.c_b_g2.serialize_compressed(&mut common_bytes).unwrap();
    let s = hash_to_field::<E>(common_bytes.clone());

    // 并行计算 s^i
    let s_pows: Vec<E::ScalarField> = (0..=ring_size_max)
        .scan(E::ScalarField::one(), |state, _| {
            let current = *state;
            *state *= s;
            Some(current)
        })
        .collect();

    let c_s_g1 = E::G1::generator().mul(r_s)
        + sma_crs.crs_g1s[ring_size_max + 1 - signer_index] * s_pows[signer_index];

    c_s_g1.serialize_compressed(&mut common_bytes).unwrap();

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

    let start_pi = Instant::now();
    let mut eq_exp_values: Vec<E::ScalarField> =
        vec![E::ScalarField::zero(); 2 * ring_size_max + 1];

    for i in 1..=ring_size_max {
        // Updating `eq_exp_values` based on the Python code logic
        *eq_exp_values.get_mut(ring_size_max + 1 - i).unwrap() += comm.r_b * s_pows[i] * t_pows[i];
        *eq_exp_values
            .get_mut(ring_size_max + 1 - i + signer_index)
            .unwrap() += s_pows[i] * t_pows[i];
        *eq_exp_values.get_mut(i).unwrap() -= r_s * t_pows[i];
        *eq_exp_values
            .get_mut(ring_size_max + 1 + i - signer_index)
            .unwrap() -= s_pows[signer_index] * t_pows[i];
    }

    // Compute pi_b, showing that bin_vec is a binary vector
    let mut b_exp_values: Vec<E::ScalarField> = vec![E::ScalarField::zero(); 2 * ring_size_max + 1];

    *b_exp_values.get_mut(0).unwrap() += r_s * comm.r_b;
    for i in 1..=ring_size_max {
        if i != signer_index {
            *b_exp_values.get_mut(ring_size_max + 1 - i).unwrap() -= comm.r_b * s_pows[i];
        }
    }

    *b_exp_values.get_mut(signer_index).unwrap() += r_s;
    for j in 1..=ring_size_max {
        *b_exp_values
            .get_mut(ring_size_max + 1 - j + signer_index)
            .unwrap() -= s_pows[j];
    }

    // Compute pi_o, showing that bin_vec contains only one 1
    let mut o_exp_values: Vec<E::ScalarField> = vec![E::ScalarField::zero(); 2 * ring_size_max + 1];

    for i in 1..=ring_size_real {
        *o_exp_values.get_mut(ring_size_max + 1 - i).unwrap() += comm.r_b;
        *o_exp_values
            .get_mut(ring_size_max + 1 - i + signer_index)
            .unwrap() += E::ScalarField::one();
    }

    // Compute pi_d2, ristrict the generation of C without using (g_(n+2), ..., g_(2n))
    let mut d2_exp_values: Vec<E::ScalarField> = vec![E::ScalarField::zero(); 2 * ring_size_max + 1];
    d2_exp_values[ring_size_max-1] = comm.r;
    d2_exp_values[ring_size_max] = ring[signer_index];

    // Compute pi_d1, showing that v_hat commit to a vector that contains 0 in its last ring_size_max-1 coordinates
    let mut d1_exp_values: Vec<E::ScalarField> = vec![E::ScalarField::zero(); 2 * ring_size_max + 1];

    for i in 2..=ring_size_max {
        *d1_exp_values.get_mut(ring_size_max + 1 - i).unwrap() += comm.r * u_pows[i];
        *d1_exp_values.get_mut(ring_size_max + 2 - i).unwrap() += ring[signer_index] * u_pows[i];
    }

    // Compute pi_phi, showing that <bin_vec, ring> = signer_hash
    let mut phi_exp_values: Vec<E::ScalarField> =
        vec![E::ScalarField::zero(); 2 * ring_size_max + 1];

    // Initialize phi_exp_values based on the logic from Python
    phi_exp_values.insert(ring_size_max, -comm.r);
    for i in 1..=ring_size_real {
        *phi_exp_values.get_mut(ring_size_max + 1 - i).unwrap() += comm.r_b * ring[i];
        *phi_exp_values
            .get_mut(ring_size_max + 1 - i + signer_index)
            .unwrap() += ring[i];
    }

    let mut pi_exps = vec![E::ScalarField::zero(); 2 * ring_size_max + 1];
    for i in 0..=2 * ring_size_max {
        pi_exps[i] = eq_exp_values[i] * delta_eq
            + b_exp_values[i] * delta_b
            + o_exp_values[i] * delta_o
            + d2_exp_values[i] * delta_d2
            + d1_exp_values[i] * delta_d1
            + phi_exp_values[i] * delta_phi;
    }

    let pi_exps_iter = cfg_iter!(pi_exps)
        .map(|w| w.into_bigint())
        .collect::<Vec<_>>();
    let pi = E::G1::msm_bigint(&sma_crs.crs_g1s, pi_exps_iter.as_slice());

    let duration_pi = start_pi.elapsed();
    records.insert(
        "Gen pi",
        format!(
            "{}.{:09} seconds",
            duration_pi.as_secs(),
            duration_pi.subsec_nanos()
        ),
    );

    let start_outsource_compute = Instant::now();
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

    let poly_1_iter = cfg_iter!(poly_1)
        .map(|w| w.into_bigint())
        .collect::<Vec<_>>();
    let poly_2_iter = cfg_iter!(poly_2)
        .map(|w| w.into_bigint())
        .collect::<Vec<_>>();
    let poly_3_iter = cfg_iter!(poly_3)
        .map(|w| w.into_bigint())
        .collect::<Vec<_>>();
    let c_h_g1 = E::G1::msm_bigint(&sma_crs.crs_g1s, poly_1_iter.as_slice());
    let c_u_g2 = E::G2::msm_bigint(&sma_crs.crs_g2s, poly_2_iter.as_slice());
    let c_t_g2 = E::G2::msm_bigint(&sma_crs.crs_g2s, poly_3_iter.as_slice());

    let duration_outsource_compute = start_outsource_compute.elapsed();
    records.insert(
        "Outsource compute",
        format!(
            "{}.{:09} seconds",
            duration_outsource_compute.as_secs(),
            duration_outsource_compute.subsec_nanos()
        ),
    );

    // Generate random scalars for outsourced computation
    let start_randomness_outsource_proof = Instant::now();
    let z = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        pi.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_h = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        c_h_g1.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_u = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        c_u_g2.serialize_compressed(&mut bytes).unwrap();
        bytes
    });
    let delta_t = hash_to_field::<E>({
        let mut bytes = common_bytes.clone();
        c_t_g2.serialize_compressed(&mut bytes).unwrap();
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

    let start_outsource_proof = Instant::now();
    // Evaluate the polynomials at r_1, r_2, r_3 respectively
    let eval_1 = kzg_evaluate::<E>(&poly_1, z);
    let eval_2 = kzg_evaluate::<E>(&poly_2, z);
    let eval_3 = kzg_evaluate::<E>(&poly_3, z);

    // Update poly_1, poly_2, poly_3 by subtracting eval results
    poly_1[0] -= eval_1;
    poly_2[0] -= eval_2;
    poly_3[0] -= eval_3;

    let mut poly_psi_1 = vec![E::ScalarField::zero(); ring_size_max + 1];
    let mut poly_psi_2 = vec![E::ScalarField::zero(); ring_size_max + 1];
    let mut poly_psi_3 = vec![E::ScalarField::zero(); ring_size_max + 1];

    // Perform polynomial division: (poly(x) - poly(u)) / (x - u)
    let divisor_1 = E::ScalarField::one();

    // For poly_psi_1 and poly_1 with divisor -r_1
    let divisor_0 = E::ScalarField::zero() - z;
    for i in 0..ring_size_max {
        poly_psi_1[ring_size_max - i - 1] = poly_1[ring_size_max - i];
        poly_1[ring_size_max - i] -= poly_psi_1[ring_size_max - i - 1] * divisor_1;
        poly_1[ring_size_max - i - 1] -= poly_psi_1[ring_size_max - i - 1] * divisor_0;
    }

    // For poly_psi_2 and poly_2 with divisor -r_2
    let divisor_0 = E::ScalarField::zero() - z;
    for i in 0..ring_size_max {
        poly_psi_2[ring_size_max - i - 1] = poly_2[ring_size_max - i];
        poly_2[ring_size_max - i] -= poly_psi_2[ring_size_max - i - 1] * divisor_1;
        poly_2[ring_size_max - i - 1] -= poly_psi_2[ring_size_max - i - 1] * divisor_0;
    }

    // For poly_psi_3 and poly_3 with divisor -r_3
    let divisor_0 = E::ScalarField::zero() - z;
    for i in 0..ring_size_max {
        poly_psi_3[ring_size_max - i - 1] = poly_3[ring_size_max - i];
        poly_3[ring_size_max - i] -= poly_psi_3[ring_size_max - i - 1] * divisor_1;
        poly_3[ring_size_max - i - 1] -= poly_psi_3[ring_size_max - i - 1] * divisor_0;
    }

    // combine
    let mut poly_pi_kzg = vec![E::ScalarField::zero(); ring_size_max + 1];
    for i in 0..=ring_size_max {
        poly_pi_kzg[i] = poly_psi_1[i] * delta_h
            + poly_psi_2[i] * delta_u
            + poly_psi_3[i] * delta_t;
    }

    // Calculate proof evaluations
    let poly_pi_kzg_iter = cfg_iter!(poly_pi_kzg)
        .map(|w| w.into_bigint())
        .collect::<Vec<_>>();

    let pi_kzg = E::G1::msm_bigint(&sma_crs.crs_g1s, poly_pi_kzg_iter.as_slice());

    let duration_outsource_proof = start_outsource_proof.elapsed();
    records.insert(
        "Outsource proof",
        format!(
            "{}.{:09} seconds",
            duration_outsource_proof.as_secs(),
            duration_outsource_proof.subsec_nanos()
        ),
    );

    let duration_proof_no_opt = duration_pi;
    records.insert(
        "Gen proof no opt",
        format!(
            "{}.{:09} seconds",
            duration_proof_no_opt.as_secs(),
            duration_proof_no_opt.subsec_nanos()
        ),
    );

    let duration_proof_opt = duration_outsource_compute + duration_pi + duration_outsource_proof;
    records.insert(
        "Gen proof opt",
        format!(
            "{}.{:09} seconds",
            duration_proof_opt.as_secs(),
            duration_proof_opt.subsec_nanos()
        ),
    );
    // println!("===========================================PROOF===============================================================");
    // for (key, value) in records.iter() {
    //     println!("{:<30}: {}", key, value);
    // }

    SmaProof {
        c_b_g2: comm.c_b_g2.into(),
        c_s_g1: c_s_g1.into(),
        pi: pi.into(),
        c_h_g1: c_h_g1.into(),
        c_u_g2: c_u_g2.into(),
        c_t_g2: c_t_g2.into(),
        pi_kzg: pi_kzg.into(),
    }
}
