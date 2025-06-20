use crate::cc;
use crate::link::PESubspaceSnark;
use crate::link::SubspaceSnark;
use crate::lrs::structures::LrsPVKey;
use crate::lrs::structures::VerifyTime;
use crate::lrs::Signature;
use crate::sma::verify_set_member_proof_opt;
use ark_ec::pairing::Pairing;
use ark_ec::CurveGroup;
use ark_ff::Field;
use ark_ff::PrimeField;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;
use ark_std::time::Instant;

pub fn verify<E>(
    lrs_pvkey: &LrsPVKey<E>,
    ring: &Vec<E::ScalarField>,
    message: &str,
    signature: &Signature<E>,
    verify_time: &mut VerifyTime,
) -> bool
where
    E: Pairing,
    <E as Pairing>::ScalarField: Field + FromStr,
    <E::ScalarField as FromStr>::Err: Debug,
{
    let verify_start = Instant::now();

    let instance = signature
        .instance
        .iter()
        .map(|x| x.into_bigint())
        .collect::<Vec<_>>();

    let cc_start = Instant::now();
    let mut result =
        cc::verify_proof(&lrs_pvkey.crs_cc.vk, &signature.cc_proof, &instance).unwrap();
    verify_time.cc = cc_start.elapsed();

    let sma_start = Instant::now();
    verify_set_member_proof_opt(
        message,
        &lrs_pvkey.crs_sma,
        &signature.sma_comm,
        &ring,
        &signature.sma_proof,
    );
    verify_time.sma = sma_start.elapsed();

    let link_start = Instant::now();
    let commitments = vec![signature.sma_comm.c_g1.into_affine(), signature.cc_proof.d];
    result = result
        && PESubspaceSnark::<E>::verify(
            &lrs_pvkey.crs_link.pp,
            &lrs_pvkey.crs_link.vk,
            &commitments,
            &signature.link_proof,
        );
    verify_time.link = link_start.elapsed();

    verify_time.verify = verify_start.elapsed();

    assert!(result, "Verification failed");
    result
}
