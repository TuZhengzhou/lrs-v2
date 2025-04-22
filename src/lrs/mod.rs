pub mod lrs_circ;
pub mod setup;
pub mod sign;
pub mod structures;
pub mod utils;
pub mod verify;

pub use structures::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use crate::lrs::read_witness_file;
    use crate::lrs::CircDescriptor;
    use crate::lrs::Constraints;
    use crate::lrs::SignTime;
    use crate::lrs::VerifyTime;
    use crate::constants::*;

    #[test]
    fn test_read_witness_file() {
        use ark_bls12_381::Fr as ScalarField;
        let filename = &format!(
            "{}/{}_js/witness.wtns.json", PATH_PREFIX_LRS_SE, CIRCUIT_NAME_LRS_SE
        );
        let _ = read_witness_file::<ScalarField>(filename).unwrap();
    }

    #[test]
    fn test_read_constraints_file() {
        use ark_bls12_381::Fr as ScalarField;
        let filename = &format!(
            "{}/{}_constraints.json", PATH_PREFIX_LRS_SE, CIRCUIT_NAME_LRS_SE
        );
        let constraints = Constraints::<ScalarField>::read_from_file(filename).unwrap();
        println!("{:?}", constraints.constraints[0]);
    }

    use crate::lrs::read_sym_file;
    #[test]
    fn test_read_sym_file() {
        let filename = &format!(
            "{}/{}.sym", PATH_PREFIX_LRS_SE, CIRCUIT_NAME_LRS_SE
        );
        let sym_data = read_sym_file(filename).unwrap();
        println!("{:?}", sym_data);
    }

    use crate::lrs::lrs_circ::*;

    #[test]
    fn test_lrs_circ() {
        use ark_bls12_381::Fr as ScalarField;

        let circ_desc = CircDescriptor {
            num_pub_io: NUM_PUB_IO_LRS_SE,
            num_commit_witness: NUM_COMMIT_WITNESS_LRS_SE,
            ioputs_name: IOPUTS_NAME_LRS_SE.iter().map(|s| s.to_string()).collect(),
            path_prefix: PATH_PREFIX_LRS_SE.to_string(),
            circuit_name: CIRCUIT_NAME_LRS_SE.to_string(),
        };
        
        let lrs_circ = LRSCirc::<ScalarField>::construct(&circ_desc).unwrap();
        println!("{:?}", lrs_circ);
    }

    #[test]
    fn test_lrs_1() {
        use crate::lrs::setup;
        use crate::lrs::sign;
        use crate::lrs::verify;
        use crate::sma::ring_gen;

        let security_par = "128".to_string();
        let msg = "test message";
        let ring_size_max = 1 << 10;

        let circ_desc = CircDescriptor {
            num_pub_io: NUM_PUB_IO_LRS_SE,
            num_commit_witness: NUM_COMMIT_WITNESS_LRS_SE,
            ioputs_name: IOPUTS_NAME_LRS_SE.iter().map(|s| s.to_string()).collect(),
            path_prefix: PATH_PREFIX_LRS_SE.to_string(),
            circuit_name: CIRCUIT_NAME_LRS_SE.to_string(),
        };

        let (lrs_pvkey, circuit) = setup::setup::<ark_bn254::Bn254>(security_par, ring_size_max, &circ_desc);

        let signer_idx = 1usize;
        let rng = &mut ark_std::test_rng();
        let mut ring = ring_gen::<_, ark_bn254::Bn254>(ring_size_max, signer_idx, rng);
        ring[signer_idx] = circuit.commit_witness[0];

        let mut sign_time = SignTime::new();
        let signature = sign::sign::<ark_bn254::Bn254>(&lrs_pvkey, &circuit, &ring, msg, &mut sign_time);

        let mut verify_time = VerifyTime::new();
        let result =
            verify::verify::<ark_bn254::Bn254>(&lrs_pvkey, &ring, msg, &signature, &mut verify_time);

        assert!(result, "Signature verification failed");
    }
}
