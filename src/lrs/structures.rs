use crate::cc::Proof;
use crate::sma::SmaComm;
use crate::sma::SmaProof;
use crate::{cc, link, sma};
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_std::fs;
#[allow(non_snake_case)]
use serde::Deserialize;
use serde_json;
use serde_json::Result as JsonResult;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use ark_std::ops::Div;

#[derive(Debug, Clone)] 
pub struct SignTime {
    pub sign: std::time::Duration,
    pub sma: std::time::Duration,
    pub cc: std::time::Duration,
    pub link: std::time::Duration,
}

impl SignTime {
    pub fn new() -> Self {
        SignTime {
            sign: std::time::Duration::new(0, 0),
            sma: std::time::Duration::new(0, 0),
            cc: std::time::Duration::new(0, 0),
            link: std::time::Duration::new(0, 0),
        }
    }
    
}

impl Div<u32> for SignTime {
    type Output = SignTime;

    /// we assue rhs greater than 0
    fn div(self, rhs: u32) -> Self::Output {
        let mut ret = self.clone();
        ret.cc /= rhs;
        ret.sma /= rhs;
        ret.link /= rhs;
        ret.sign /= rhs;
        
        ret
    }
}

#[derive(Debug, Clone)] 
pub struct VerifyTime {
    pub verify: std::time::Duration,
    pub sma: std::time::Duration,
    pub cc: std::time::Duration,
    pub link: std::time::Duration,
}

impl VerifyTime {
    pub fn new() -> Self {
        VerifyTime {
            verify: std::time::Duration::new(0, 0),
            sma: std::time::Duration::new(0, 0),
            cc: std::time::Duration::new(0, 0),
            link: std::time::Duration::new(0, 0),
        }
    }
}

impl Div<u32> for VerifyTime {
    type Output = VerifyTime;

    /// we assue rhs greater than 0
    fn div(self, rhs: u32) -> Self::Output {
        let mut ret = self.clone();
        ret.cc /= rhs;
        ret.sma /= rhs;
        ret.link /= rhs;
        ret.verify /= rhs;
        
        ret
    }
    
}


#[derive(Debug, Clone)] 
pub struct CircDescriptor {
    pub num_pub_io: usize,
    pub num_commit_witness: usize,
    pub ioputs_name: Vec<String>,
    pub path_prefix: String,
    pub circuit_name: String,
}

#[derive(Clone)]
pub struct LrsPVKey<E: Pairing>
where
    <E as Pairing>::ScalarField: Field + FromStr,
    <E::ScalarField as FromStr>::Err: Debug,
{
    pub crs_link: link::LinkEVKey<E::G1Affine, E::G2Affine, E::ScalarField>,
    pub crs_cc: cc::CcPVKey<E>,
    pub crs_sma: sma::SmaCRS<E>,
}

pub struct Signature<E: Pairing> {
    pub sma_comm: SmaComm<E>,
    pub sma_proof: SmaProof<E>,
    pub cc_proof: Proof<E>,
    pub link_proof: Vec<E::G1Affine>,
    pub instance: Vec<E::ScalarField>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConstraintsRaw {
    constraints: Vec<Vec<HashMap<usize, String>>>,
}

// 顶层结构体（泛型）
#[derive(Debug, Clone)]
pub struct Constraints<F>
where
    F: Field + FromStr,
    <F as FromStr>::Err: Debug,
{
    pub constraints: Vec<Vec<HashMap<usize, F>>>,
}

impl<F> Constraints<F>
where
    F: Field + FromStr,
    <F as FromStr>::Err: Debug,
{
    pub fn read_from_file(path: &str) -> JsonResult<Constraints<F>> {
        let json_str = fs::read_to_string(path).map_err(serde_json::Error::io)?;
        let constraints_raw: ConstraintsRaw = serde_json::from_str(&json_str)?;

        let mut constraints = Vec::new();
        for constraint in &constraints_raw.constraints {
            let mut a: HashMap<usize, F> = HashMap::new();
            for term in &constraint[0] {
                a.insert(term.0.clone(), F::from_str(term.1.as_str()).unwrap());
            }

            let mut b: HashMap<usize, F> = HashMap::new();
            for term in &constraint[1] {
                b.insert(term.0.clone(), F::from_str(term.1.as_str()).unwrap());
            }

            let mut c: HashMap<usize, F> = HashMap::new();
            for term in &constraint[2] {
                c.insert(term.0.clone(), F::from_str(term.1.as_str()).unwrap());
            }

            constraints.push(vec![a, b, c]);
        }

        Ok(Constraints { constraints })
    }
}
