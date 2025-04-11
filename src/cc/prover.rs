use crate::cc::{helpers::hash_to_field, r1cs_to_qap::R1CStoQAP, Proof, ProvingKey};
use crate::link::scalar_vector_mult;
use ark_ec::{pairing::Pairing, AffineRepr, CurveGroup, VariableBaseMSM};
use ark_ff::Field;
use ark_ff::{PrimeField, UniformRand, Zero};
use ark_poly::GeneralEvaluationDomain;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, OptimizationGoal, Result as R1CSResult,
};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::Rng;
use ark_std::{cfg_iter, end_timer, start_timer, vec::Vec};
use std::ops::Mul;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Create a Groth16 proof that is zero-knowledge.
/// This method samples randomness for zero knowledges via `rng`.
pub fn create_random_proof<E, C, R>(
    circuit: C,
    pk: &ProvingKey<E>,
    rng: &mut R,
) -> R1CSResult<(Proof<E>, Vec<E::ScalarField>, E::ScalarField)>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
    R: Rng,
{
    let r_a = E::ScalarField::rand(rng);
    let r_b = E::ScalarField::rand(rng);
    let xi = E::ScalarField::rand(rng);
    let v = E::ScalarField::rand(rng);

    create_proof::<E, C>(circuit, pk, r_a, r_b, xi, v)
        .map(|(proof, committed_witnesses)| (proof, committed_witnesses, v))
}

/// Create a Groth16 proof using randomness `r_a` and `r_b`.
#[inline]
pub fn create_proof<E, C>(
    circuit: C,
    pk: &ProvingKey<E>,
    r_a: E::ScalarField,
    r_b: E::ScalarField,
    xi: E::ScalarField,
    v: E::ScalarField,
) -> R1CSResult<(Proof<E>, Vec<E::ScalarField>)>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
{
    type D<F> = GeneralEvaluationDomain<F>;

    let prover_time = start_timer!(|| "Groth16::Prover");
    let cs = ConstraintSystem::new_ref();

    // Set the optimization goal
    cs.set_optimization_goal(OptimizationGoal::Constraints);

    // Synthesize the circuit.
    let synthesis_time = start_timer!(|| "Constraint synthesis");
    circuit.generate_constraints(cs.clone())?; // Drives generation of new constraints inside cs
    debug_assert!(cs.is_satisfied().unwrap());
    end_timer!(synthesis_time);

    // Finalize the constraint system (either by outlining or inlining, if an optimization goal is set).
    let lc_time = start_timer!(|| "Inlining LCs");
    cs.finalize();
    end_timer!(lc_time);

    let witness_map_time = start_timer!(|| "R1CS to QAP witness map");
    let h = R1CStoQAP::witness_map::<E::ScalarField, D<E::ScalarField>>(cs.clone())?;
    end_timer!(witness_map_time);

    // Prepare Information
    let prover = cs.borrow().unwrap();
    // let num_instance_vars = prover.num_instance_variables();
    // let num_witness_vars = prover.num_witness_variables();
    let instance_assignment_with_one_field = cfg_iter!(prover.instance_assignment)
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>();

    let instance_assignment = instance_assignment_with_one_field[1..].to_vec();

    let witness_assignment = cfg_iter!(prover.witness_assignment)
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>();

    let committed_witnesses = &witness_assignment[..pk.commit_witness_num];

    let mut uncommitted_witnesses = witness_assignment[pk.commit_witness_num..].to_vec();

    let assignment = [&instance_assignment[..], &witness_assignment[..]].concat();

    drop(prover);
    drop(cs);

    let delta_g1_prime = pk.delta_g1.mul(xi);
    let delta_g2_prime = pk.delta_g2.mul(xi);

    // Compute [A]_1
    let a_acc_time = start_timer!(|| "Compute A");
    let g_a = calculate_coeff::<E::G1Affine>(
        delta_g1_prime.mul(r_a),
        &pk.a_query,
        pk.alpha_g1,
        &assignment,
    );
    end_timer!(a_acc_time);

    // Compute [B]_1
    let b_g1_acc_time = start_timer!(|| "Compute B in G1");
    let g1_b = calculate_coeff(
        delta_g1_prime.mul(r_b),
        &pk.b_g1_query,
        pk.beta_g1,
        &assignment,
    );
    end_timer!(b_g1_acc_time);

    // Compute [B]_2
    let b_g2_acc_time = start_timer!(|| "Compute B in G2");
    let g2_b = calculate_coeff(
        delta_g2_prime.mul(r_b),
        &pk.b_g2_query,
        pk.beta_g2,
        &assignment,
    );
    end_timer!(b_g2_acc_time);
    drop(assignment);

    // Compute [D]_1
    let d_acc_time = start_timer!(|| "Compute D");
    let mut g_d = pk.eta_gamma_inv_g1.mul(v);
    g_d += E::G1::msm_bigint(&pk.comm_wit_abc_query, &committed_witnesses);
    end_timer!(d_acc_time);

    // Compute [C]_1
    let mut bytes = Vec::new();
    g_a.serialize_compressed(&mut bytes).unwrap();
    g2_b.serialize_compressed(&mut bytes).unwrap();
    g_d.serialize_compressed(&mut bytes).unwrap();
    let m_hash = hash_to_field::<E>(bytes);

    let inv_xi_m_hash = (xi + m_hash).inverse().unwrap();
    let s_a: E::ScalarField = xi * r_a * inv_xi_m_hash;
    let s_b: E::ScalarField = xi * r_b * inv_xi_m_hash;

    assert!(h[h.len() - 1] == E::ScalarField::zero()); // last element is always zero
    let h_assignment = scalar_vector_mult::<E>(&inv_xi_m_hash, &h, h.len());
    let h_assignment = h_assignment
        .iter()
        .map(|s| s.into_bigint())
        .collect::<Vec<_>>();

    let h_acc = E::G1::msm_bigint(&pk.h_query, &h_assignment);
    drop(h_assignment);
    drop(h);

    for i in 0..uncommitted_witnesses.len() {
        uncommitted_witnesses[i] = (inv_xi_m_hash
            * E::ScalarField::from_bigint(uncommitted_witnesses[i]).unwrap())
        .into_bigint();
    }

    let ucomm_wit_abc_acc = E::G1::msm_bigint(&pk.ucomm_wit_abc_query, &uncommitted_witnesses);
    drop(uncommitted_witnesses);

    let c_time: ark_std::perf_trace::TimerInfo = start_timer!(|| "Finish C");
    let mut g_c = E::G1::zero();
    g_c += &ucomm_wit_abc_acc;
    g_c += &h_acc;
    g_c += g_a.mul(s_b);
    g_c += g1_b.mul(s_a);
    g_c -= &pk.delta_g1.mul(s_a * s_b * (xi + m_hash));
    g_c -= &pk.eta_delta_inv_g1.mul(v * inv_xi_m_hash);
    end_timer!(c_time);
    end_timer!(prover_time);

    Ok((
        Proof {
            a: g_a.into_affine(),
            b: g2_b.into_affine(),
            c: g_c.into_affine(),
            d: g_d.into_affine(),
            delta_prime: delta_g2_prime.into_affine(),
        },
        committed_witnesses
            .iter()
            .map(|b| E::ScalarField::from_bigint(*b).unwrap())
            .collect(),
    ))
}

fn calculate_coeff<G: AffineRepr>(
    initial: G::Group,
    query: &[G],
    vk_param: G,
    assignment: &[<G::ScalarField as PrimeField>::BigInt],
) -> G::Group {
    let acc = G::Group::msm_bigint(&query[1..], assignment);
    initial + query[0] + acc + vk_param
}
