use ark_ec::{pairing::Pairing, Group};
use ark_ec::{scalar_mul::fixed_base::FixedBase, AffineRepr, CurveGroup};
use ark_ff::Field;
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_std::rand::RngCore;
use ark_std::UniformRand;

use super::SmaCRS;

#[allow(non_snake_case)]
pub fn crs_key_gen<R: RngCore, E: Pairing>(
    _security_par: String,
    ring_size_max: usize,
    rng: &mut R,
) -> SmaCRS<E> {
    let mut crs_g1s = Vec::with_capacity(2 * ring_size_max + 1);
    let mut crs_g2s = Vec::with_capacity(ring_size_max + 1);

    let alpha = E::ScalarField::rand(rng);
    let alpha_exps = (0..(ring_size_max * 2 + 1))
        .map(|i| alpha.pow(&[i as u64]))
        .collect::<Vec<_>>();

    let window_size = FixedBase::get_mul_window_size(ring_size_max * 2 + 1);
    let scalar_size =
        <<E as Pairing>::G1Affine as AffineRepr>::ScalarField::MODULUS_BIT_SIZE as usize;
    let outerc = (scalar_size + window_size - 1) / window_size;
    let g1_table = FixedBase::get_window_table(
        scalar_size,
        window_size,
        E::G1::generator(),
    );

    crs_g1s.push(E::G1::generator().into());
    for i in 1..=(2 * ring_size_max) {
        crs_g1s.push(FixedBase::windowed_mul::<E::G1>(
            outerc,
            window_size,
            &g1_table,
            &alpha_exps[i],
        ).into_affine());
    }
    crs_g1s[ring_size_max + 1] = E::G1::zero().into_affine();

    let window_size = FixedBase::get_mul_window_size(ring_size_max);
    let scalar_size =
        <<E as Pairing>::G1Affine as AffineRepr>::ScalarField::MODULUS_BIT_SIZE as usize;
    let outerc = (scalar_size + window_size - 1) / window_size;
    let g2_table = FixedBase::get_window_table(
        scalar_size,
        window_size,
        E::G2::generator(),
    );

    crs_g2s.push(E::G2::generator().into());
    for i in 1..(ring_size_max + 1) {
        crs_g2s.push(FixedBase::windowed_mul::<E::G2>(
            outerc,
            window_size,
            &g2_table,
            &alpha_exps[i],
        ).into_affine());
    }

    SmaCRS {
        g1_generator: E::G1::generator(),
        g2_generator: E::G2::generator(),
        crs_g1s,
        crs_g2s,
        ring_size_max,
    }
}
