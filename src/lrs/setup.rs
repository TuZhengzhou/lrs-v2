use crate::cc;
use crate::cc::CcPVKey;
use crate::link;
use crate::link::snark::SubspaceSnark;
use crate::link::SparseMatrix;
use crate::lrs::lrs_circ::LRSCirc;
use crate::lrs::structures::LrsPVKey;
use crate::sma;
use ark_ec::pairing::Pairing;
use ark_ec::CurveGroup;
use ark_ec::Group;
use ark_relations::r1cs::Field;
use ark_std::fmt::Debug;
use ark_std::str::FromStr;

use super::CircDescriptor;

pub fn setup<E>(
    security_par: String,
    ring_size_max: usize,
    circ_desc: &CircDescriptor,
) -> (LrsPVKey<E>, LRSCirc<E::ScalarField>)
where
    E: Pairing,
    <E as Pairing>::ScalarField: Field + FromStr,
    <E::ScalarField as FromStr>::Err: Debug,
{
    let rng = &mut ark_std::test_rng();

    let lrs_circ = LRSCirc::<E::ScalarField>::construct(circ_desc).unwrap();

    let crs_sma = sma::crs_key_gen(security_par, ring_size_max, rng);

    let crs_cc: CcPVKey<E> =
        cc::generate_random_parameters(lrs_circ.clone(), circ_desc.num_commit_witness, rng)
            .unwrap();

    let link_rows = 2;
    let link_cols = circ_desc.num_commit_witness + 2;
    let link_pp = link::PP {
        nr: link_rows,
        nc: link_cols,
        g1: E::G1::generator().into_affine(),
        g2: E::G2::generator().into_affine(),
    };

    let mut link_m = SparseMatrix::<E::G1Affine>::new(link_rows, link_cols);
    link_m.insert_row_slice(0, 0, vec![crs_sma.crs_g1s[1], crs_sma.crs_g1s[0]]);
    link_m.insert_row_slice(1, 0, crs_cc.pk.comm_wit_abc_query.clone());
    assert!(crs_cc.pk.comm_wit_abc_query.len() == circ_desc.num_commit_witness);
    link_m.insert_row_slice(
        1,
        circ_desc.num_commit_witness + 1,
        vec![crs_cc.pk.eta_gamma_inv_g1],
    );

    let crs_link = link::PESubspaceSnark::<E>::keygen(rng, &link_pp, &link_m);

    // Return the generated CRS
    (
        LrsPVKey {
            crs_link,
            crs_cc,
            crs_sma,
        },
        lrs_circ,
    )
}
