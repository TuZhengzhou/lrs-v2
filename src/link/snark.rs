use crate::link::matrix::*;
use ark_ec::pairing::Pairing;
use ark_ec::AffineRepr;
use ark_ec::CurveGroup;
use ark_ff::UniformRand;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{cfg_iter, rand::Rng, vec::Vec};
use std::ops::Mul;
use ark_std::Zero;
use std::ops::Neg;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct PP<
    G1: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    G2: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
> {
    pub nr: usize, // # of rows
    pub nc: usize, // # of cols
    pub g1: G1,
    pub g2: G2,
}

impl<
        G1: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
        G2: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    > PP<G1, G2>
{
    pub fn new(nr: usize, nc: usize, g1: &G1, g2: &G2) -> PP<G1, G2> {
        PP {
            nr,
            nc,
            g1: g1.clone(),
            g2: g2.clone(),
        }
    }
}

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct EK<
    G1: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    ScalarField: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
> {
    pub tau: ScalarField,  // tau \in Z_p
    pub p_0: Vec<Vec<G1>>, // [C_0]_1 \in G_1^(nr \times (k+1)), k = 1
    pub p_1: Vec<Vec<G1>>, // [C_1]_1 \in G_1^(nr \times (k+1)), k = 1
}

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct VK<
    G2: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    ScalarField: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
> {
    pub tau: ScalarField, // tau \in Z_p
    pub c_0: Vec<G2>,     // [C_0]_1 \in G_2^(nr \times (k)), k = 1
    pub c_1: Vec<G2>,     // [C_1]_1 \in G_2^(nr \times (k)), k = 1
    pub a: Vec<G2>,       // [A]_1 \in G_2^(k+1 \times k), k = 1
}

#[derive(Clone, Default, PartialEq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct LinkEVKey<
    G1: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    G2: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
    ScalarField: Clone + Default + CanonicalSerialize + CanonicalDeserialize,
> {
    pub pp: PP<G1, G2>,
    pub ek: EK<G1, ScalarField>,
    pub vk: VK<G2, ScalarField>,
}

pub trait SubspaceSnark {
    type KMtx;
    type InVec;
    type OutVec;

    type PP;

    type EK;
    type VK;
    type EVK;

    type Proof;

    fn keygen<R: Rng>(rng: &mut R, pp: &Self::PP, m: &Self::KMtx) -> Self::EVK;
    fn prove(pp: &Self::PP, ek: &Self::EK, x: &[Self::InVec]) -> Self::Proof;
    fn verify(pp: &Self::PP, vk: &Self::VK, y: &[Self::OutVec], pi: &Self::Proof) -> bool;
}

fn vec_to_g2<PE: Pairing>(
    pp: &PP<PE::G1Affine, PE::G2Affine>,
    v: &Vec<PE::ScalarField>,
) -> Vec<PE::G2Affine> {
    v.iter()
        .map(|x| pp.g2.mul(*x).into_affine())
        .collect::<Vec<_>>()
}

pub struct PESubspaceSnark<PE: Pairing> {
    pairing_engine_type: std::marker::PhantomData<PE>,
}

// NB: Now the system is for y = Mx
impl<PE: Pairing> SubspaceSnark for PESubspaceSnark<PE> {
    type KMtx = SparseMatrix<PE::G1Affine>;
    type InVec = PE::ScalarField;
    type OutVec = PE::G1Affine;

    type PP = PP<PE::G1Affine, PE::G2Affine>;

    type EK = EK<PE::G1Affine, PE::ScalarField>;
    type VK = VK<PE::G2Affine, PE::ScalarField>;
    type EVK = LinkEVKey<PE::G1Affine, PE::G2Affine, PE::ScalarField>;

    type Proof = Vec<PE::G1Affine>;

    fn keygen<R: Rng>(rng: &mut R, pp: &Self::PP, m: &Self::KMtx) -> Self::EVK {
        // K_0, K_1 \in Z_p^(nr \times k+1), k = 1
        let mut k_0_c0: Vec<PE::ScalarField> = Vec::with_capacity(pp.nr);
        let mut k_0_c1: Vec<PE::ScalarField> = Vec::with_capacity(pp.nr);
        let mut k_1_c0: Vec<PE::ScalarField> = Vec::with_capacity(pp.nr);
        let mut k_1_c1: Vec<PE::ScalarField> = Vec::with_capacity(pp.nr);
        for _ in 0..pp.nr {
            k_0_c0.push(PE::ScalarField::rand(rng));
            k_0_c1.push(PE::ScalarField::rand(rng));
            k_1_c0.push(PE::ScalarField::rand(rng));
            k_1_c1.push(PE::ScalarField::rand(rng));
        }

        // println!("m: {:?}", m);

        // [A]_1 \in G_2^(k+1 \times k), k = 1
        let mut a: Vec<PE::ScalarField> = Vec::with_capacity(2);
        a.push(PE::ScalarField::rand(rng));
        a.push(PE::ScalarField::rand(rng));

        // M \in Z_p^(nr \times nc)
        // P_0 = M^T K_0 \in Z_p^(nc \time k+1)
        // P_1 = M^T K_1 \in Z_p^(nc \time k+1)
        let p_0_c0 = SparseLinAlgebra::<PE>::sparse_vector_matrix_mult(&k_0_c0, &m, pp.nc);
        let p_0_c1 = SparseLinAlgebra::<PE>::sparse_vector_matrix_mult(&k_0_c1, &m, pp.nc);
        let p_0: Vec<Vec<PE::G1Affine>> = vec![p_0_c0.clone(), p_0_c1.clone()];

        let p_1_c0 = SparseLinAlgebra::<PE>::sparse_vector_matrix_mult(&k_1_c0, &m, pp.nc);
        let p_1_c1 = SparseLinAlgebra::<PE>::sparse_vector_matrix_mult(&k_1_c1, &m, pp.nc);
        let p_1: Vec<Vec<PE::G1Affine>> = vec![p_1_c0.clone(), p_1_c1.clone()];

        // for i in 0..pp.nc {
        //     assert!(p_0_c0[i] == pp.g1.mul(k_0_c0[0]).into_affine());
        //     assert!(p_0_c1[i] == pp.g1.mul(k_0_c1[0]).into_affine());
        //     assert!(p_1_c0[i] == pp.g1.mul(k_1_c0[0]).into_affine());
        //     assert!(p_1_c1[i] == pp.g1.mul(k_1_c1[0]).into_affine());
        // }
        
        // C_0 = K_0 \times A \in G_1^(nr \times k)
        // C_1 = K_1 \times A \in G_1^(nr \times k)
        let mut c_0: Vec<PE::ScalarField> = Vec::with_capacity(pp.nr);
        let mut c_1: Vec<PE::ScalarField> = Vec::with_capacity(pp.nr);
        for i in 0..pp.nr {
            c_0.push(k_0_c0[i].mul(a[0]) + k_0_c1[i].mul(a[1]));
            c_1.push(k_1_c0[i].mul(a[0]) + k_1_c1[i].mul(a[1]));
        }

        // tau \in Z_p
        let tau: <PE as Pairing>::ScalarField = PE::ScalarField::rand(rng);

        let ek = EK::<PE::G1Affine, PE::ScalarField> { tau, p_0, p_1 };
        
        let vk = VK::<PE::G2Affine, PE::ScalarField> {
            tau,
            c_0: vec_to_g2::<PE>(pp, &c_0),
            c_1: vec_to_g2::<PE>(pp, &c_1),
            a: vec_to_g2::<PE>(pp, &a),
        };

        let evk = LinkEVKey::<PE::G1Affine, PE::G2Affine, PE::ScalarField> {
            pp: pp.clone(),
            ek,
            vk,
        };
        evk
    }

    fn prove(pp: &Self::PP, ek: &Self::EK, x: &[Self::InVec]) -> Self::Proof {
        assert_eq!(pp.nc, x.len());

        // p_0 \in G_1^(nc \times (k+1)), k = 1
        // p_1 \in G_1^(nc \times (k+1)), k = 1
        // p = p_0 + \tau * p_1 \in G_1^(nc \times (k+1)), k = 1
        let mut p_c0: Vec<PE::G1Affine> = Vec::with_capacity(pp.nc);
        let mut p_c1: Vec<PE::G1Affine> = Vec::with_capacity(pp.nc);
        for i in 0..pp.nc {
            let i0 = ek.p_0[0][i] + ek.p_1[0][i].mul(ek.tau);
            let i1 = ek.p_0[1][i] + ek.p_1[1][i].mul(ek.tau);
            p_c0.push(i0.into_affine());
            p_c1.push(i1.into_affine());
        };
        // let p: Vec<Vec<PE::G1Affine>> = vec![p_c0, p_c1];

        // pi = x ^T * (p_0 + \tau * p_1) \in G_1^(k+1), k = 1
        let mut pi_0 = PE::G1::zero();
        let mut pi_1 = PE::G1::zero();
        for j in 0..pp.nc {
            pi_0 = pi_0 + &p_c0[j].mul(x[j]);
            pi_1 = pi_1 + &p_c1[j].mul(x[j]);
        }
        let pi = vec![pi_0.into_affine(), pi_1.into_affine()];

        pi
    }

    fn verify(pp: &Self::PP, vk: &Self::VK, y: &[Self::OutVec], pi: &Self::Proof) -> bool {
        assert_eq!(pp.nr, y.len());

        let mut left = y.to_vec();
        left.push(pi[0]);
        left.push(pi[1]);

        // C_0 \in G_2^(nr \times k), k = 1
        // C_1 \in G_2^(nr \times k), k = 1
        // C = C_0 + \tau * C_1 \in G_2^(nr \times k), k = 1
        let mut c: Vec<PE::G2Affine> = Vec::with_capacity(pp.nr);
        for i in 0..pp.nr {
            c.push(PE::G2Affine::from(vk.c_0[i] + vk.c_1[i].mul(vk.tau)));
        }
        let mut right = c.to_vec();
        right.push(PE::G2Affine::from(vk.a[0].into_group().neg()));
        right.push(PE::G2Affine::from(vk.a[1].into_group().neg()));

        
        let result = PE::multi_pairing(
            cfg_iter!(left)
                .map(|x| x.clone())
                .collect::<Vec<_>>()
                .as_slice(),
            cfg_iter!(right)
                .map(|x| x.clone())
                .collect::<Vec<_>>()
                .as_slice(),
        ).is_zero();

        result
    }
}

