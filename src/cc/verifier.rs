use crate::cc::helpers::hash_to_field;
use crate::cc::{Proof, VerifyingKey};
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;

use ark_relations::r1cs::SynthesisError;
use ark_ec::{CurveGroup, VariableBaseMSM};
use std::ops::Neg;

/// Verify a Groth16 proof `proof` against the prepared verification key `vk`,
/// with respect to the instance `public_inputs`.
pub fn verify_proof<E: Pairing>(
    vk: &VerifyingKey<E>,
    proof: &Proof<E>,
    instance: &[<E::ScalarField as PrimeField>::BigInt],
) -> crate::Result<bool> {
    // Compute [C]_1
    let mut bytes = Vec::new();
    proof.a.serialize_compressed(&mut bytes).unwrap();
    proof.b.serialize_compressed(&mut bytes).unwrap();
    proof.d.serialize_compressed(&mut bytes).unwrap();
    let m_hash = hash_to_field::<E>(bytes);

    let instance_acc = vk.instance_abc_query[0] + E::G1::msm_bigint(&vk.instance_abc_query[1..], instance);

    let qap = E::multi_miller_loop(
        [
            <E::G1Prepared>::from(proof.a),
            <E::G1Prepared>::from(proof.c),
            <E::G1Prepared>::from(proof.d),
            <E::G1Prepared>::from(instance_acc),
        ],
        [
            proof.b.into(),
            (vk.delta_g2_neg_pc * m_hash - proof.delta_prime).into(),
            vk.gamma_g2_neg_pc.into(),
            vk.g2_generator.neg().into_affine()
        ],
    );

    let test = E::final_exponentiation(qap).ok_or(SynthesisError::UnexpectedIdentity)?;
    let result = test == vk.alpha_beta_gt;
    assert!(result, "Verification failed");

    Ok(result)
}
