pub mod error;
pub mod generator;
pub mod helpers;
pub mod prover;
pub mod r1cs_to_qap;
pub mod structures;
pub mod verifier;
pub mod utils;

pub use error::*;
pub use generator::*;
pub use prover::*;
pub use structures::*;
pub use verifier::*;

#[cfg(test)]
pub mod tests {

    use crate::cc::utils::*;

    #[test]
    fn test_cc_prove_and_verify_1() {
        cc_prove_and_verify_1::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_2() {
        cc_prove_and_verify_2::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_3() {
        cc_prove_and_verify_3::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_merkle() {
        cc_prove_and_verify_merkle::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_phi() {
        cc_prove_and_verify_phi::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_schnorr_sign() {
        cc_prove_and_verify_schnorr_sign::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_schnorr_verify() {
        cc_prove_and_verify_schnorr_verify::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_lrs() {
        cc_prove_and_verify_lrs::<ark_bn254::Bn254>(1);
    }

    #[test]
    fn test_cc_prove_and_verify_lrs_a() {
        cc_prove_and_verify_lrs_a::<ark_bn254::Bn254>(1);
    }
}
