use super::lrs_circ::LRSCirc;
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
) -> Signature<E>
where
    E: Pairing,
    <E as Pairing>::ScalarField: Field + FromStr,
    <E::ScalarField as FromStr>::Err: Debug,
{
    let rng = &mut ark_std::test_rng();
    let (cc_proof, mut comm_witness, v) =
        cc::create_random_proof(circuit.clone(), &lrs_pvkey.crs_cc.pk, rng).unwrap();

    let comm = sma::commit::<_, E>(&ring, &lrs_pvkey.crs_sma, 1, rng);
    let sma_proof = sma::set_member_proof_opt(message, &lrs_pvkey.crs_sma, &comm, ring, 1, rng);

    comm_witness.push(comm.r);
    comm_witness.push(v);

    let link_proof = PESubspaceSnark::<E>::prove(
        &lrs_pvkey.crs_link.pp,
        &lrs_pvkey.crs_link.ek,
        &comm_witness,
    );

    Signature {
        sma_comm: comm,
        sma_proof,
        cc_proof,
        link_proof,
        instance: circuit.instance.clone(),
    }
}
