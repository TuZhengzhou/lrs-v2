use super::{create_random_proof, generate_random_parameters, verify_proof, CcPVKey};
use crate::cc;
use crate::constants::*;
use crate::lrs::lrs_circ::LRSCirc;
use crate::lrs::structures::CircDescriptor;
use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use ark_relations::lc;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystemRef;
use ark_relations::r1cs::Field;
use ark_relations::r1cs::SynthesisError;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;
use ark_std::UniformRand;
use std::ops::MulAssign;

pub fn cc_prove_and_verify<E>(circ_desc: &CircDescriptor, lrs_circ: &LRSCirc<E::ScalarField>)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let rng = &mut ark_std::test_rng();

    let crs_cc: CcPVKey<E> =
        cc::generate_random_parameters(lrs_circ.clone(), circ_desc.num_commit_witness, rng)
            .unwrap();

    let (cc_proof, _, _) = cc::create_random_proof(lrs_circ.clone(), &crs_cc.pk, rng).unwrap();

    let instance = lrs_circ
        .instance
        .iter()
        .map(|v| v.into_bigint())
        .collect::<Vec<<E::ScalarField as ark_ff::PrimeField>::BigInt>>();
    let result = verify_proof(&crs_cc.vk, &cc_proof, &instance);

    assert!(result.is_ok());
}

pub struct TestCircuit1<F: Field> {
    a: Option<F>,
    b: Option<F>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for TestCircuit1<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| {
            let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;

            a.mul_assign(&b);
            Ok(a)
        })?;

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;

        Ok(())
    }
}

/// 1 public variable, 2 uncommitted witness variables
pub fn cc_prove_and_verify_1<E>(n_iters: usize)
where
    E: Pairing,
{
    let rng = &mut ark_std::test_rng();
    let params: CcPVKey<E> =
        generate_random_parameters::<E, _, _>(TestCircuit1 { a: None, b: None }, 0, rng).unwrap();

    for _ in 0..n_iters {
        let a = E::ScalarField::rand(rng);
        let b = E::ScalarField::rand(rng);
        let mut c = a;
        c.mul_assign(&b);

        // Create a LegoGro16 proof with our parameters.
        let (proof, _, _) = create_random_proof(
            TestCircuit1 {
                a: Some(a),
                b: Some(b),
            },
            &params.pk,
            rng,
        )
        .unwrap();

        let instance = vec![c.into_bigint()];
        verify_proof(&params.vk, &proof, &instance).unwrap();
    }
}

pub struct TestCircuit2<F: Field> {
    a: Option<F>,
    b: Option<F>,
    e: Option<F>,
    f: Option<F>,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for TestCircuit2<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let e = cs.new_witness_variable(|| self.e.ok_or(SynthesisError::AssignmentMissing))?;
        let f = cs.new_witness_variable(|| self.f.ok_or(SynthesisError::AssignmentMissing))?;

        let c = cs.new_input_variable(|| {
            let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;
            a.mul_assign(&b);
            Ok(a)
        })?;

        let d = cs.new_input_variable(|| {
            let mut e = self.e.ok_or(SynthesisError::AssignmentMissing)?;
            let f = self.f.ok_or(SynthesisError::AssignmentMissing)?;
            e.mul_assign(&f);
            Ok(e)
        })?;

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        cs.enforce_constraint(lc!() + e, lc!() + f, lc!() + d)?;
        cs.enforce_constraint(lc!() + e, lc!() + f, lc!() + d)?;

        Ok(())
    }
}

/// 2 public inputs, 4 uncommitted witness
pub fn cc_prove_and_verify_2<E>(n_iters: usize)
where
    E: Pairing,
{
    let rng = &mut ark_std::test_rng();
    let params: CcPVKey<E> = generate_random_parameters::<E, _, _>(
        TestCircuit2 {
            a: None,
            b: None,
            e: None,
            f: None,
        },
        0,
        rng,
    )
    .unwrap();

    for _ in 0..n_iters {
        let a = E::ScalarField::rand(rng);
        let b = E::ScalarField::rand(rng);
        let e = E::ScalarField::rand(rng);
        let f = E::ScalarField::rand(rng);
        let c = a * b;
        let d = e * f;

        // Create a LegoGro16 proof with our parameters.
        let (proof, _, _) = create_random_proof(
            TestCircuit2 {
                a: Some(a),
                b: Some(b),
                e: Some(e),
                f: Some(f),
            },
            &params.pk,
            rng,
        )
        .unwrap();

        let instance = vec![c.into_bigint(), d.into_bigint()];
        verify_proof(&params.vk, &proof, &instance).unwrap();
    }
}

/// 2 public inputs, 2 committed witness, 2 uncommitted witness
pub fn cc_prove_and_verify_3<E>(n_iters: usize)
where
    E: Pairing,
{
    let rng = &mut ark_std::test_rng();
    let params: CcPVKey<E> = generate_random_parameters::<E, _, _>(
        TestCircuit2 {
            a: None,
            b: None,
            e: None,
            f: None,
        },
        2,
        rng,
    )
    .unwrap();

    for _ in 0..n_iters {
        let a = E::ScalarField::rand(rng);
        let b = E::ScalarField::rand(rng);
        let e = E::ScalarField::rand(rng);
        let f = E::ScalarField::rand(rng);
        let c = a * b;
        let d = e * f;

        // Create a LegoGro16 proof with our parameters.
        let (proof, _, _) = create_random_proof(
            TestCircuit2 {
                a: Some(a),
                b: Some(b),
                e: Some(e),
                f: Some(f),
            },
            &params.pk,
            rng,
        )
        .unwrap();

        let instance = vec![c.into_bigint(), d.into_bigint()];
        verify_proof(&params.vk, &proof, &instance).unwrap();
    }
}

pub fn cc_prove_and_verify_merkle<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_MERKLE,
        num_commit_witness: NUM_COMMIT_WITNESS_MERKLE,
        ioputs_name: IOPUTS_NAME_MERKLE.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_MERKLE.to_string(),
        circuit_name: CIRCUIT_NAME_MERKLE.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}

pub fn cc_prove_and_verify_phi<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let num_pub_io = 1usize;
    let num_commit_witness = 1usize;
    let ioputs_name = vec!["main.phi", "main.sk"];
    let path_prefix = "./circoms/tests/phi/";
    let circuit_name = "phi";

    let circ_desc = CircDescriptor {
        num_pub_io,
        num_commit_witness,
        ioputs_name: ioputs_name.iter().map(|s| s.to_string()).collect(),
        path_prefix: path_prefix.to_string(),
        circuit_name: circuit_name.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}

pub fn cc_prove_and_verify_schnorr_sign<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_COMMIT_WITNESS_SCHNORR_SIGN,
        num_commit_witness: NUM_COMMIT_WITNESS_SCHNORR_SIGN,
        ioputs_name: IOPUTS_NAME_SCHNORR_SIGN
            .iter()
            .map(|s| s.to_string())
            .collect(),
        path_prefix: PATH_PREFIX_SCHNORR_SIGN.to_string(),
        circuit_name: CIRCUIT_NAME_SCHNORR_SIGN.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}

pub fn cc_prove_and_verify_schnorr_verify<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_SCHNORR_VERIFY,
        num_commit_witness: NUM_COMMIT_WITNESS_SCHNORR_VERIFY,
        ioputs_name: IOPUTS_NAME_SCHNORR_VERIFY
            .iter()
            .map(|s| s.to_string())
            .collect(),
        path_prefix: PATH_PREFIX_SCHNORR_VERIFY.to_string(),
        circuit_name: CIRCUIT_NAME_SCHNORR_VERIFY.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}

pub fn cc_prove_and_verify_lrs<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_LRS_SE,
        num_commit_witness: NUM_COMMIT_WITNESS_LRS_SE,
        ioputs_name: IOPUTS_NAME_LRS_SE.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_LRS_SE.to_string(),
        circuit_name: CIRCUIT_NAME_LRS_SE.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}

pub fn cc_prove_and_verify_lrs_a<E>(n_iters: usize)
where
    E: Pairing,
    <<E as Pairing>::ScalarField as FromStr>::Err: Debug,
{
    let circ_desc = CircDescriptor {
        num_pub_io: NUM_PUB_IO_LRS_A,
        num_commit_witness: NUM_COMMIT_WITNESS_LRS_A,
        ioputs_name: IOPUTS_NAME_LRS_A.iter().map(|s| s.to_string()).collect(),
        path_prefix: PATH_PREFIX_LRS_A.to_string(),
        circuit_name: CIRCUIT_NAME_LRS_A.to_string(),
    };

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(&circ_desc).unwrap();

    for _ in 0..n_iters {
        cc_prove_and_verify::<E>(&circ_desc, &lrs_circ);
    }
}
