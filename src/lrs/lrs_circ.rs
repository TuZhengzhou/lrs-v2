use crate::lrs::structures::*;
use crate::lrs::utils::*;
use ark_ff::Field;
use ark_relations::r1cs::Variable;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct LRSCirc<F>
where
    F: Field + FromStr,
    <F as FromStr>::Err: Debug,
{
    pub constraints: Constraints<F>,
    pub sym_data: HashMap<usize, String>,
    pub witness_data: HashMap<usize, F>,
    pub instance: Vec<F>,
    pub commit_witness: Vec<F>,
    pub circ_desc: CircDescriptor,
}

impl<F: Field + FromStr> LRSCirc<F>
where
    <F as FromStr>::Err: Debug,
{
    // -> Result<LRSCirc<F>, SynthesisError>
    pub fn construct(circ_desc: &CircDescriptor) -> Result<LRSCirc<F>, SynthesisError> {
        // public inputs | committed witness | uncommitted witness
        let ioputs_name = circ_desc.ioputs_name
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<String>>();

        let constraints = Constraints::<F>::from(
            Constraints::read_from_file(
                (circ_desc.path_prefix.clone() + &circ_desc.circuit_name + "_constraints.json").as_str(),
            )
            .expect(&format!("Failed to read {}_constraints.json", &circ_desc.circuit_name)),
        );

        let witness_data: HashMap<usize, F> =
            read_witness_file((circ_desc.path_prefix.clone() + &circ_desc.circuit_name + "_js/witness.wtns.json").as_str())
                .expect(&format!(
                    "Failed to read {}_js/witness.wtns.json",
                    &circ_desc.circuit_name
                ));

        let sym_data: HashMap<usize, String> =
            read_sym_file((circ_desc.path_prefix.clone() + &circ_desc.circuit_name + ".sym").as_str())
                .expect(&format!("Failed to read {}.sym", &circ_desc.circuit_name));

        let instance = ioputs_name[0..circ_desc.num_pub_io]
            .iter()
            .map(|name| {
                let idx = sym_data
                    .iter()
                    .find(|(_, n)| *n == name)
                    .map(|(index, _)| *index)
                    .ok_or(SynthesisError::AssignmentMissing)?;
                let value = witness_data
                    .get(&idx)
                    .ok_or(SynthesisError::AssignmentMissing)?;
                Ok(*value)
            })
            .collect::<Result<Vec<F>, SynthesisError>>()?;
        // println!("instance: {:?}", instance);

        let commit_witness = ioputs_name[circ_desc.num_pub_io..(circ_desc.num_pub_io + circ_desc.num_commit_witness)]
            .iter()
            .map(|name| {
                let idx = sym_data
                    .iter()
                    .find(|(_, n)| *n == name)
                    .map(|(index, _)| *index)
                    .ok_or(SynthesisError::AssignmentMissing)?;
                let value = witness_data
                    .get(&idx)
                    .ok_or(SynthesisError::AssignmentMissing)?;
                Ok(*value)
            })
            .collect::<Result<Vec<F>, SynthesisError>>()?;
        // println!("committed_witness: {:?}", commit_witness);

        // Get input witness data
        Ok(LRSCirc {
            constraints,
            sym_data,
            witness_data,
            instance,
            commit_witness,
            circ_desc: circ_desc.clone(),
        })
    }
}

impl<F: Field + FromStr> ConstraintSynthesizer<F> for LRSCirc<F>
where
    <F as FromStr>::Err: Debug,
{
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // 1. Load circuit.sym and find the value of main.phi
        let sym_data = self.sym_data.clone();
        let witness_data = self.witness_data.clone();

        let mut variables: HashMap<usize, Variable> = HashMap::new();

        // Make sure commit_witness are the first witnesses variable created
        for i in 0..(self.circ_desc.num_pub_io + self.circ_desc.num_commit_witness) {
            let var_name = &self.circ_desc.ioputs_name[i];
            let idx = sym_data
                .iter()
                .find(|(_, name)| *name == var_name)
                .map(|(index, _)| *index)
                .ok_or(SynthesisError::AssignmentMissing)?;

            let value = witness_data
                .get(&idx)
                .ok_or(SynthesisError::AssignmentMissing)?;

            if i < self.circ_desc.num_pub_io {
                let comm_var = cs.new_input_variable(|| Ok(*value))?;
                variables.insert(idx, comm_var);
            } else {
                // Create commit witness variables
                let comm_var = cs.new_witness_variable(|| Ok(*value))?;
                variables.insert(idx, comm_var);
            }
            // println!("var_name: {:?}, idx: {:?}, value: {:?}", var_name, idx, value);
        }

        // 2. Create other witnesses and public input variables (such as main.out)
        for entry in self.sym_data.iter() {
            let idx = *entry.0;

            // skip the commit witness and public input variables
            let var_name = entry.1;

            let mut skip = false;
            for i in 0..(self.circ_desc.num_pub_io + self.circ_desc.num_commit_witness) {
                if var_name == &self.circ_desc.ioputs_name[i] {
                    // println!(
                    //     "Skipping commit witness or public input variable: {:?}, idx: {:?}",
                    //     var_name, idx
                    // );
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }

            // Create other witness variables
            let witness_value = witness_data
                .get(&idx)
                .ok_or(SynthesisError::AssignmentMissing)?;

          
            // Create other witness variables
            let witness_var = cs.new_witness_variable(|| Ok(*witness_value))?;
            variables.insert(idx, witness_var);
            
        }

        // 3. Parse and apply all constraints in circuit.json
        for (index, constraint) in self.constraints.constraints.iter().enumerate() {
            let a_comb = parse_linear_comb(&constraint[0], &variables)?;
            let b_comb = parse_linear_comb(&constraint[1], &variables)?;
            let c_comb = parse_linear_comb(&constraint[2], &variables)?;

            if let Err(e) = cs.enforce_constraint(a_comb, b_comb, c_comb) {
                println!("Error enforcing constraint #{}: {:?}", index, e);
                return Err(e);
            }
        }

        Ok(())
    }
}
