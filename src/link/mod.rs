mod matrix;
pub mod snark;

pub use matrix::*;
pub use snark::*;

#[cfg(test)]
mod tests {
    use super::{PESubspaceSnark, SparseMatrix, SubspaceSnark, PP};
    // use ark_bls12_381::{Bn254, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
    use ark_bn254::{Bn254, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
    use ark_ec::{CurveGroup, Group};
    use ark_ff::{One, Zero};
    use ark_std::rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_basic_g2() {
        // Prove knowledge of all `x_i` in `y = \sum_i g_i * x_i`
        let mut rng = StdRng::seed_from_u64(0u64);
        let g1 = G1Projective::generator().into_affine();
        let g2 = G2Projective::generator().into_affine();

        let mut pp = PP::<G1Affine, G2Affine> { nr: 2, nc: 3, g1, g2 };

        let mut m = SparseMatrix::new(2, 3);
        m.insert_row_slice(0, 0, vec![g1, g1, g1]);
        m.insert_row_slice(1, 0, vec![g1, g1, g1]);

        let x: Vec<Fr> = vec![Fr::zero(), Fr::one(), Fr::zero()];

        let x_bad: Vec<Fr> = vec![Fr::one(), Fr::one(), Fr::one()];

        let y: Vec<G1Affine> = vec![g1, g1];

        let evk = PESubspaceSnark::<Bn254>::keygen(&mut rng, &pp, &m);

        let pi = PESubspaceSnark::<Bn254>::prove(&mut pp, &evk.ek, &x);
        let pi_bad = PESubspaceSnark::<Bn254>::prove(&mut pp, &evk.ek, &x_bad);

        assert!(PESubspaceSnark::<Bn254>::verify(&pp, &evk.vk, &y, &pi) == true);
        assert!(PESubspaceSnark::<Bn254>::verify(&pp, &evk.vk, &y, &pi_bad) == false);
    }
}
