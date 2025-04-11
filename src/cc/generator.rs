use crate::cc::{r1cs_to_qap::R1CStoQAP, ProvingKey, VerifyingKey, CcPVKey};
use ark_ec::{pairing::Pairing, scalar_mul::fixed_base::FixedBase, CurveGroup, Group};
use ark_ff::{Field, PrimeField, UniformRand, Zero};
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, OptimizationGoal, Result as R1CSResult,
    SynthesisError, SynthesisMode,
};
use ark_std::rand::Rng;
use ark_std::{cfg_into_iter, cfg_iter, start_timer, end_timer};
use std::ops::Mul;
use std::ops::Neg;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Generates a random common reference string for
/// a circuit.
pub fn generate_random_parameters<E, C, R>(
    circuit: C,
    commit_witness_count: usize,
    rng: &mut R,

) -> R1CSResult<CcPVKey<E>>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
    R: Rng,
{
    let alpha = E::ScalarField::rand(rng);
    let beta = E::ScalarField::rand(rng);
    let delta = E::ScalarField::rand(rng);
    let eta = E::ScalarField::rand(rng);
    let gamma = E::ScalarField::rand(rng);

    generate_parameters::<E, C, R>(circuit, commit_witness_count, alpha, beta, delta, eta, gamma, rng)
}

/// Create parameters for a circuit, given some toxic waste.
pub fn generate_parameters<E, C, R>(
    circuit: C,
    commit_witness_count: usize,
    alpha: E::ScalarField,
    beta: E::ScalarField,
    delta: E::ScalarField,
    eta: E::ScalarField,
    gamma: E::ScalarField,
    rng: &mut R,
) -> R1CSResult<CcPVKey<E>>
where
    E: Pairing,
    C: ConstraintSynthesizer<E::ScalarField>,
    R: Rng,
{
    type D<F> = GeneralEvaluationDomain<F>;

    let setup_time = start_timer!(|| "Groth16::Generator");
    let cs = ConstraintSystem::new_ref();
    cs.set_optimization_goal(OptimizationGoal::Constraints);
    cs.set_mode(SynthesisMode::Setup);

    // Synthesize the circuit.
    let synthesis_time = start_timer!(|| "Constraint synthesis");
    circuit.generate_constraints(cs.clone())?;
    end_timer!(synthesis_time);

    let lc_time = start_timer!(|| "Inlining LCs");
    cs.finalize();
    end_timer!(lc_time);

    ///////////////////////////////////////////////////////////////////////////
    let domain_time = start_timer!(|| "Constructing evaluation domain");

    let domain_size = cs.num_constraints() + cs.num_instance_variables();
    let domain = D::new(domain_size).ok_or(SynthesisError::PolynomialDegreeTooLarge)?;
    let t = domain.sample_element_outside_domain(rng);

    end_timer!(domain_time);
    ///////////////////////////////////////////////////////////////////////////
    let num_instance = cs.num_instance_variables();
    // let num_witness = cs.num_witness_variables();

    let reduction_time = start_timer!(|| "R1CS to QAP Instance Map with Evaluation");
    let (a, b, c, zt, qap_num_variables, m_raw) =
        R1CStoQAP::instance_map_with_evaluation::<E::ScalarField, D<E::ScalarField>>(cs, &t)?;
    end_timer!(reduction_time);

    // Compute query densities
    let non_zero_a: usize = cfg_into_iter!(0..qap_num_variables)
        .map(|i| usize::from(!a[i].is_zero()))
        .sum();

    let non_zero_b: usize = cfg_into_iter!(0..qap_num_variables)
        .map(|i| usize::from(!b[i].is_zero()))
        .sum();

    let scalar_bits = E::ScalarField::MODULUS_BIT_SIZE as usize;

    let gamma_inverse = gamma.inverse().ok_or(SynthesisError::UnexpectedIdentity)?;
    let delta_inverse = delta.inverse().ok_or(SynthesisError::UnexpectedIdentity)?;

    let g1_generator = E::G1::generator();
    let g2_generator = E::G2::generator();

    // Compute G2 window table
    let g2_time = start_timer!(|| "Compute G2 table");
    let g2_window = FixedBase::get_mul_window_size(non_zero_b);
    let g2_table =
        FixedBase::get_window_table::<E::G2>(scalar_bits, g2_window, g2_generator);
    end_timer!(g2_time);

    // Compute the B-query in G2
    let b_g2_time = start_timer!(|| "Calculate B G2");
    let b_g2_query =
        FixedBase::msm::<E::G2>(scalar_bits, g2_window, &g2_table, &b);
    drop(g2_table);
    end_timer!(b_g2_time);

    // Compute G1 window table
    let g1_window_time = start_timer!(|| "Compute G1 window table");
    let g1_window =
        FixedBase::get_mul_window_size(non_zero_a + non_zero_b + qap_num_variables + m_raw + 1);
    let g1_table =
        FixedBase::get_window_table::<E::G1>(scalar_bits, g1_window, g1_generator);
    end_timer!(g1_window_time);

    // Generate the R1CS proving key
    let proving_key_time = start_timer!(|| "Generate the R1CS proving key");

    // Compute the B-query in G1
    let b_g1_time = start_timer!(|| "Calculate B G1");
    let b_g1_query =
        FixedBase::msm::<E::G1>(scalar_bits, g1_window, &g1_table, &b);
    end_timer!(b_g1_time);

    // Compute the A-query
    let a_time = start_timer!(|| "Calculate A");
    let a_query =
        FixedBase::msm::<E::G1>(scalar_bits, g1_window, &g1_table, &a);
    
    end_timer!(a_time);

    // Compute the H-query
    let h_time = start_timer!(|| "Calculate H");
    let h_query = FixedBase::msm::<E::G1>(
        scalar_bits,
        g1_window,
        &g1_table,
        &cfg_into_iter!(0..m_raw - 1)
            .map(|i| zt * &delta_inverse * &t.pow([i as u64]))
            .collect::<Vec<_>>(),
    );
    end_timer!(h_time);

    // Compute the instance query
    let instance_abc = cfg_iter!(a[..num_instance])
        .zip(&b[..num_instance])
        .zip(&c[..num_instance])
        .map(|((a, b), c)| (beta * a + &(alpha * b) + c))
        .collect::<Vec<_>>();
    let instance_abc_query = FixedBase::msm::<E::G1>(
        scalar_bits,
        g1_window,
        &g1_table,
        &instance_abc,
    );
    drop(instance_abc);

    // Compute the commit witness query
    let comm_wit_abc = cfg_iter!(a[(num_instance)..(num_instance+commit_witness_count)])
    .zip(&b[(num_instance)..(num_instance+commit_witness_count)])
    .zip(&c[(num_instance)..(num_instance+commit_witness_count)])
    .map(|((a, b), c)| (beta * a + &(alpha * b) + c) * &gamma_inverse)
    .collect::<Vec<_>>();

    let comm_wit_abc_query = FixedBase::msm::<E::G1>(
        scalar_bits,
        g1_window,
        &g1_table,
        &comm_wit_abc,
    );
    drop(comm_wit_abc);

    // Compute the uncommitted witness query
    let ucomm_wit_abc = cfg_iter!(a[(num_instance+commit_witness_count)..])
        .zip(&b[(num_instance+commit_witness_count)..])
        .zip(&c[(num_instance+commit_witness_count)..])
        .map(|((a, b), c)| (beta * a + &(alpha * b) + c) * &delta_inverse)
        .collect::<Vec<_>>();
    drop(b);
    drop(a);
    drop(c);

    let ucomm_wit_abc_time = start_timer!(|| "Calculate L");
    let ucomm_wit_abc_query = FixedBase::msm::<E::G1>(
        scalar_bits,
        g1_window,
        &g1_table,
        &ucomm_wit_abc,
    );
    drop(ucomm_wit_abc);
    end_timer!(ucomm_wit_abc_time);

    let alpha_g1 = g1_generator.mul_bigint(alpha.into_bigint());
    let beta_g1 = g1_generator.mul_bigint(beta.into_bigint());
    let beta_g2 = g2_generator.mul_bigint(beta.into_bigint());
    let delta_g1 = g1_generator.mul_bigint(delta.into_bigint());
    let delta_g2 = g2_generator.mul_bigint(delta.into_bigint());
    let gamma_g2 = g2_generator.mul_bigint(gamma.into_bigint());

    end_timer!(proving_key_time);

    let batch_normalization_time = start_timer!(|| "Convert proving key elements to affine");
    let a_query = E::G1::normalize_batch(&a_query);
    let b_g1_query = E::G1::normalize_batch(&b_g1_query);
    let b_g2_query = E::G2::normalize_batch(&b_g2_query);
    let h_query = E::G1::normalize_batch(&h_query);
    let comm_wit_abc_query = E::G1::normalize_batch(&comm_wit_abc_query);
    let ucomm_wit_abc_query = E::G1::normalize_batch(&ucomm_wit_abc_query);
    let instance_abc_query = E::G1::normalize_batch(&instance_abc_query);
    end_timer!(batch_normalization_time);

    let pk = ProvingKey {
        g1_generator,
        g2_generator,
        alpha_g1: E::G1Affine::from(alpha_g1),
        beta_g1: E::G1Affine::from(beta_g1),
        beta_g2: E::G2Affine::from(beta_g2),
        delta_g1: E::G1Affine::from(delta_g1),
        delta_g2: E::G2Affine::from(delta_g2),
        eta_delta_inv_g1: E::G1Affine::from(g1_generator.mul(eta * delta_inverse)),
        eta_gamma_inv_g1: E::G1Affine::from(g1_generator.mul(eta * gamma_inverse)),
        a_query,
        b_g1_query,
        b_g2_query,
        h_query,
        comm_wit_abc_query,
        ucomm_wit_abc_query,
        commit_witness_num: commit_witness_count,
    };

    // Generate R1CS verification key
    let verifying_key_time = start_timer!(|| "Generate the R1CS verification key");

    drop(g1_table);
    end_timer!(verifying_key_time);

    let vk = VerifyingKey::<E> {
        g1_generator,
        g2_generator,
        alpha_beta_gt: E::pairing(alpha_g1, beta_g2),
        delta_g2_neg_pc: delta_g2.neg().into_affine().into(),
        gamma_g2_neg_pc: gamma_g2.neg().into_affine().into(),
        instance_abc_query,
        commit_witness_num: commit_witness_count,
    };
    end_timer!(setup_time);

    Ok( CcPVKey { pk, vk } )
}
