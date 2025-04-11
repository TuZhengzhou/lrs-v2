use ark_ec::pairing::{Pairing, PairingOutput};
use ark_serialize::*;
use ark_std::vec::Vec;

/// Modifying Paper `Simulation Extractable Versions of Groth’s zk-SNARK Revisited` to the commit-carry version

/// A proof in the Groth16 SNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof<E: Pairing> {
    /// The `A` element in `G1`.
    pub a: E::G1Affine,
    /// The `B` element in `G2`.
    pub b: E::G2Affine,
    /// The `C` element in `G1`.
    pub c: E::G1Affine,
    /// The `D` element in `G1`.
    pub d: E::G1Affine,
    /// The `delta_prime` element in `G2`.
    pub delta_prime: E::G2Affine,
}

impl<E: Pairing> Default for Proof<E> {
    fn default() -> Self {
        Self {
            a: E::G1Affine::default(),
            b: E::G2Affine::default(),
            c: E::G1Affine::default(),
            d: E::G1Affine::default(),
            delta_prime: E::G2Affine::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// A verification key in the Groth16 SNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct VerifyingKey<E: Pairing> {
    pub g1_generator: E::G1,
    pub g2_generator: E::G2,
    /// The e(`alpha * G`, `beta * H`) element in `E::GT`.
    pub alpha_beta_gt: PairingOutput<E>,
    pub delta_g2_neg_pc: E::G2Affine,
    pub gamma_g2_neg_pc: E::G2Affine,
    pub instance_abc_query: Vec<E::G1Affine>,
    /// The num of witness to commit
    pub commit_witness_num: usize,
}

impl<E: Pairing> Default for VerifyingKey<E> {
    fn default() -> Self {
        Self {
            g1_generator: E::G1::default(),
            g2_generator: E::G2::default(),
            alpha_beta_gt: PairingOutput::<E>::default(),
            delta_g2_neg_pc: E::G2Affine::default(),
            gamma_g2_neg_pc: E::G2Affine::default(),
            instance_abc_query: Vec::new(),
            commit_witness_num: 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

/// The prover key for for the Groth16 zkSNARK.
#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct ProvingKey<E: Pairing> {
    pub g1_generator: E::G1,
    pub g2_generator: E::G2,
    /// The element `alpha * G` in `E::G1`.
    pub alpha_g1: E::G1Affine,
    /// The element `beta * G` in `E::G1`.
    pub beta_g1: E::G1Affine,
    /// The element `beta * H` in `E::G2`.
    pub beta_g2: E::G2Affine,
    /// The element `delta * G` in `E::G1`.
    pub delta_g1: E::G1Affine,
    /// The element `delta * G` in `E::G2`.
    pub delta_g2: E::G2Affine,
    pub eta_delta_inv_g1: E::G1Affine,
    pub eta_gamma_inv_g1: E::G1Affine,
    /// The elements `a_i * G` in `E::G1`.
    pub a_query: Vec<E::G1Affine>,
    /// The elements `b_i * G` in `E::G1`.
    pub b_g1_query: Vec<E::G1Affine>,
    /// The elements `b_i * H` in `E::G2`.
    pub b_g2_query: Vec<E::G2Affine>,
    /// The elements `h_i * G` in `E::G1`.
    pub h_query: Vec<E::G1Affine>,
    /// The committed witness query of C
    pub comm_wit_abc_query: Vec<E::G1Affine>,
    /// The uncommitted witness query of C
    pub ucomm_wit_abc_query: Vec<E::G1Affine>,
    /// The num of witness to commit
    pub commit_witness_num: usize,
}

impl<E: Pairing> Default for ProvingKey<E> {
    fn default() -> Self {
        Self {
            g1_generator: E::G1::default(),
            g2_generator: E::G2::default(),
            alpha_g1: E::G1Affine::default(),
            beta_g1: E::G1Affine::default(),
            beta_g2: E::G2Affine::default(),
            delta_g1: E::G1Affine::default(),
            delta_g2: E::G2Affine::default(),
            eta_delta_inv_g1: E::G1Affine::default(),
            eta_gamma_inv_g1: E::G1Affine::default(),
            a_query: Vec::new(),
            b_g1_query: Vec::new(),
            b_g2_query: Vec::new(),
            h_query: Vec::new(),
            comm_wit_abc_query: Vec::new(),
            ucomm_wit_abc_query: Vec::new(),
            commit_witness_num: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct CcPVKey<E: Pairing> {
    pub pk: ProvingKey<E>,
    pub vk: VerifyingKey<E>,
}

impl <E: Pairing> Default for CcPVKey<E> {
    fn default() -> Self {
        Self {
            pk: ProvingKey::default(),
            vk: VerifyingKey::default(),
        }
    }
}
