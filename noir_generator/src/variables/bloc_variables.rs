use crate::variables::variable::Variable;
use crate::variables::var_type::VarType;
use crate::random::{self, Random};

use super::var_type;

#[derive(Clone)]
pub struct BlocVariables{
    variables: Vec<Variable>
}

impl BlocVariables {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable)
    }

    pub fn variables(&self) -> Vec<Variable> {
        self.variables.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn new_variable(&mut self, allowed_types: &VarType, mutable: bool) -> Variable{
        let new_var = Variable::new(
            format!("var{}", self.next_id()),
            mutable,
            allowed_types,
        );

        self.add_variable(new_var.clone());

        new_var
    }

    pub fn get_random_variable(&self, random: &mut Random, allowed_types: Vec<VarType>, mutable: bool) -> Option<&Variable> {
        let filtered_variables: Vec<&Variable> = self.variables
            .iter()
            .filter(|v| {
                let type_condition = allowed_types.iter().any(|allowed_type| {
                    var_type::way_to_type(random, &v.var_type(), &allowed_type).is_some()
                });
                let mutable_condition = mutable == v.is_mutable();
        
                type_condition && mutable_condition
            })
            .collect();

        if filtered_variables.len() == 0 {
            return None;
        }
    
        Some(random.choose_random_item_from_vec(&filtered_variables))
    }

    fn next_id(&self) -> usize {
        self.variables.len()+1
    }
}