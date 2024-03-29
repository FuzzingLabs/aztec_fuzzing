use crate::constants::MAX_LAMBDA_ARGUMENTS;
use crate::functions::lambda::Lambda;
use crate::variables::variable::Variable;
use crate::variables::var_type::VarType;
use crate::random::Random;

use super::list_structs::ListStructs;
use super::var_type;

#[derive(Clone)]
pub struct BlocData{
    variables: Vec<Variable>,
    lambdas: Vec<Lambda>
}

impl BlocData {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            lambdas: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.variables.push(variable)
    }

    pub fn add_lambda(&mut self, lambda: Lambda) {
        self.lambdas.push(lambda);
    }

    pub fn variables(&self) -> Vec<Variable> {
        self.variables.clone()
    }

    pub fn lambdas(&self) -> Vec<Lambda> {
        self.lambdas.clone()
    }

    pub fn is_variables_empty(&self) -> bool {
        self.variables.is_empty()
    }

    pub fn is_lambdas_empty(&self) -> bool {
        self.lambdas.is_empty()
    }

    pub fn create_lambda(&mut self, random: &mut Random, list_structs: &ListStructs, ret_type: &VarType) -> Lambda {
        let mut bloc_variables = BlocData::new();
        for _ in 0..MAX_LAMBDA_ARGUMENTS{

            bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &var_type::random_type(random, list_structs)));
        }
        
        let new_lambda = Lambda::new(
            self.next_lambda_name(),
            bloc_variables,
            Some(ret_type.clone()),
        );

        new_lambda
    }

    pub fn get_random_variable(&self, random: &mut Random, allowed_types: Vec<VarType>, mutable: bool) -> Option<&Variable> {
        let filtered_variables: Vec<&Variable> = self.variables
            .iter()
            .filter(|v| {
                let type_condition = allowed_types.iter().any(|allowed_type| {
                    var_type::way_to_type(random, &v.var_type(), &allowed_type).is_some()
                });
                let mutable_condition = !mutable | v.is_mutable();
        
                type_condition && mutable_condition
            })
            .collect();

        if filtered_variables.len() == 0 {
            return None;
        }
    
        Some(random.choose_random_item_from_vec(&filtered_variables))
    }

    pub fn get_random_lambda(&self, random: &mut Random, allowed_types: Vec<VarType>) -> Option<&Lambda> {
        let filtered_lambdas: Vec<&Lambda> = self.lambdas
            .iter()
            .filter(|l| {
                match l.ret_type() {
                    Some(t) => allowed_types.iter().any(|allowed_type| {
                            var_type::way_to_type(random, t, &allowed_type).is_some()
                        }),
                    None => false,
                }
            })
            .collect();

        if filtered_lambdas.len() == 0 {
            return None;
        }
    
        Some(random.choose_random_item_from_vec(&filtered_lambdas))
    }

    pub fn next_variable_name(&self) -> String {
        format!("var{}", self.variables.len()+1)
    }

    pub fn next_lambda_name(&self) -> String {
        format!("lambda{}", self.lambdas.len()+1)
    }

}