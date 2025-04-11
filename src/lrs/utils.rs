use ark_ff::Field;
use ark_relations::r1cs::Variable;
use ark_relations::r1cs::{LinearCombination, SynthesisError};
use ark_std::fs::File;
use ark_std::io::BufReader;
use serde_json;
use serde_json::Result as JsonResult;
#[allow(non_snake_case)]
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;

pub fn read_witness_file<F>(filename: &str) -> JsonResult<HashMap<usize, F>>
where
    F: Field + FromStr,
    <F as FromStr>::Err: Debug,
{
    let file = File::open(filename).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);
    let witness_raw: HashMap<usize, String> = serde_json::from_reader(reader)?;

    let mut witness: HashMap<usize, F> = HashMap::new();
    for (key, value) in witness_raw {
        let field_value: F = value.parse().expect("Failed to parse field element");
        witness.insert(key, field_value);
    }

    // File will be automatically closed when it goes out of scope.
    Ok(witness)
}

pub fn read_sym_file(filename: &str) -> JsonResult<HashMap<usize, String>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sym_data = HashMap::<usize, String>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 4 {
            let witness_index = parts[1].parse::<isize>().unwrap();
            let name = parts[3].to_string();

            if witness_index != -1 {
                sym_data.insert(witness_index as usize, name.clone());
            }
        }
    }

    Ok(sym_data)
}

pub fn parse_linear_comb<F>(
    terms: &HashMap<usize, F>,
    variables: &HashMap<usize, Variable>,
) -> Result<LinearCombination<F>, SynthesisError>
where
    F: Field + FromStr,
    <F as FromStr>::Err: Debug,
{
    let mut lc = LinearCombination::zero();

    for (var_name, coeff) in terms {
        let var = if *var_name == 0usize {
            ark_relations::r1cs::Variable::Instance(0)
        } else {
            variables
                .get(var_name)
                .ok_or(SynthesisError::AssignmentMissing)?
                .clone()
        };
        lc = lc + (coeff.clone(), var);
    }
    Ok(lc)
}
