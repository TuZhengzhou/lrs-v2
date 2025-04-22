use super::lrs_circ::LRSCirc;
use super::structures::SignTime;
use crate::cc;
use crate::link::PESubspaceSnark;
use crate::link::SubspaceSnark;
use crate::lrs::structures::LrsPVKey;
use crate::lrs::Signature;
use crate::sma;
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;

pub fn sign<E>(
    lrs_pvkey: &LrsPVKey<E>,
    circuit: &LRSCirc<E::ScalarField>,
    ring: &Vec<E::ScalarField>,
    message: &str,
    sign_time: &mut SignTime,
) -> Signature<E>
where
    E: Pairing,
    <E as Pairing>::ScalarField: Field + FromStr,
    <E::ScalarField as FromStr>::Err: Debug,
{
    let start = std::time::Instant::now();

    let rng = &mut ark_std::test_rng();
    let cc_start = std::time::Instant::now();
    let (cc_proof, mut comm_witness, v) =
        cc::create_random_proof(circuit.clone(), &lrs_pvkey.crs_cc.pk, rng).unwrap();
    sign_time.cc = cc_start.elapsed();

    let sma_start = std::time::Instant::now();
    let comm = sma::commit::<_, E>(&ring, &lrs_pvkey.crs_sma, 1, rng);
    let sma_proof = sma::set_member_proof_opt(message, &lrs_pvkey.crs_sma, &comm, ring, 1, rng);
    sign_time.sma = sma_start.elapsed();

    let link_start = std::time::Instant::now();
    comm_witness.push(comm.r);
    comm_witness.push(v);

    let link_proof = PESubspaceSnark::<E>::prove(
        &lrs_pvkey.crs_link.pp,
        &lrs_pvkey.crs_link.ek,
        &comm_witness,
    );
    sign_time.link = link_start.elapsed();

    sign_time.sign = start.elapsed();
    Signature {
        sma_comm: comm,
        sma_proof,
        cc_proof,
        link_proof,
        instance: circuit.instance.clone(),
    }
}
