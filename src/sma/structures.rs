use ark_ec::pairing::Pairing;
use ark_serialize::*;
use ark_std::vec::Vec;
use ark_std::Zero;

/// Modifying Paper `Simulation Extractable Versions of Groth’s zk-SNARK Revisited` to the commit-carry version

#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct SmaCRS<E: Pairing> {
    pub g1_generator: E::G1,
    pub g2_generator: E::G2,
    pub crs_g1s: Vec<E::G1Affine>,
    pub crs_g2s: Vec<E::G2Affine>,
    pub ring_size_max: usize,
}

impl <E: Pairing> Default for SmaCRS<E> {
    fn default() -> Self {
        Self {
            g1_generator: E::G1::default(),
            g2_generator: E::G2::default(),
            crs_g1s: Vec::new(),
            crs_g2s: Vec::new(),
            ring_size_max: 0,
        }
    }
    
}

#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct SmaComm<E: Pairing> {
    pub c_g1:       E::G1,
    pub c_b_g2:     E::G2,
    pub r:          E::ScalarField,
    pub r_b:        E::ScalarField,
}

impl <E: Pairing> Default for SmaComm<E> {
    fn default() -> Self {
        Self {
            c_g1:       E::G1::default(),
            c_b_g2:     E::G2::default(),
            r:          E::ScalarField::zero(),
            r_b:        E::ScalarField::zero(),
        }
    }
}

/// A proof in the Groth16 SNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct SmaProof<E: Pairing> {
    pub c_b_g2: E::G2Affine,
    pub c_s_g1: E::G1Affine,
    pub pi:     E::G1Affine,
    pub c_h_g1: E::G1Affine,
    pub c_u_g2: E::G2Affine,
    pub c_t_g2: E::G2Affine,
    pub pi_kzg: E::G1Affine,
}

impl <E: Pairing> Default for SmaProof<E> {
    fn default() -> Self {
        Self {
            c_b_g2: E::G2Affine::default(),
            c_s_g1: E::G1Affine::default(),
            pi:     E::G1Affine::default(),
            c_h_g1: E::G1Affine::default(),
            c_u_g2: E::G2Affine::default(),
            c_t_g2: E::G2Affine::default(),
            pi_kzg: E::G1Affine::default(),
        }
    }
    
}