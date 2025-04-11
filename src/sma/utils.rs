use crate::sma::{SmaCRS, SmaComm};
use ark_ec::AffineRepr;
use ark_ec::pairing::Pairing;
use ark_ff::{One, Zero};
use ark_std::fmt::Debug;
use ark_std::rand::RngCore;
use ark_std::str::FromStr;
use ark_std::UniformRand;
use std::ops::Mul;

#[allow(unused_variables)]
pub fn ring_gen<R: RngCore, E: Pairing>(
    ring_size_max: usize,
    signer_index: usize,
    rng: &mut R,
) -> Vec<E::ScalarField>
where
    E::G1Affine: AffineRepr<BaseField = E::BaseField>,
    <E::ScalarField as FromStr>::Err: Debug,
{
    let mut ring = Vec::new();
    ring.push(E::ScalarField::zero()); // 0. Takes the 0-th index
    for _ in 1..=ring_size_max {
        ring.push(E::ScalarField::rand(rng));
    }
    ring
}

pub fn commit<R: RngCore, E: Pairing>(
    ring: &Vec<E::ScalarField>,
    sma_crs: &SmaCRS<E>,
    signer_index: usize,
    rng: &mut R,
) -> SmaComm<E>
where
    E: Pairing,
{
    let r = E::ScalarField::rand(rng);
    let r_b = E::ScalarField::rand(rng);

    let c_g1 = sma_crs.g1_generator.mul(r) + sma_crs.crs_g1s[1].mul(ring[signer_index]);
    let c_b_g2 = sma_crs.g2_generator.mul(r_b) + sma_crs.crs_g2s[signer_index];

    SmaComm {
        c_g1,
        c_b_g2,
        r,
        r_b,
    }
}

pub fn kzg_evaluate<E: Pairing>(poly: &Vec<E::ScalarField>, x: E::ScalarField) -> E::ScalarField {
    let mut eval = E::ScalarField::zero();
    let mut temp = E::ScalarField::one();
    for coef in poly {
        eval += *coef * temp;
        temp *= x;
    }
    eval
}
