use crate::functions::lambda::Lambda;
use crate::tools::constants::CONFIG;
use crate::tools::random::Random;
use crate::variables::var_type::VarType;
use crate::variables::variable::Variable;

use super::list_structs::ListStructs;
use super::var_type;

/// Represent all data (every declared variable and lambda function) from a specific code block
#[derive(Clone)]
pub struct BlocData {
    variables: Vec<Variable>,
    lambdas: Vec<Lambda>,
    name_number_to_skip: usize,
}

impl BlocData {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            lambdas: Vec::new(),
            name_number_to_skip: 0,
        }
    }

    pub fn set_name_number_to_skip(&mut self, nb: usize) {
        self.name_number_to_skip = nb;
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

    /// Add a randomly generated new lambda function with a specified return type to this list
    pub fn create_lambda(
        &mut self,
        random: &mut Random,
        list_structs: &ListStructs,
        ret_type: &VarType,
    ) -> Lambda {
        let mut bloc_variables = BlocData::new();
        for _ in 0..CONFIG.max_lambda_arguments {
            bloc_variables.add_variable(Variable::new(
                bloc_variables.next_variable_name(),
                false,
                &var_type::random_type(random, list_structs),
            ));
        }

        let new_lambda = Lambda::new(
            self.next_lambda_name(),
            bloc_variables,
            Some(ret_type.clone()),
        );

        new_lambda
    }

    /// Return a randomly chosen variable that has a type matching one of the list of types given as parameter
    /// Return None if there is none
    pub fn get_random_variable(
        &self,
        random: &mut Random,
        allowed_types: Vec<&VarType>,
        mutable: bool,
    ) -> Option<&Variable> {
        let filtered_variables: Vec<&Variable> = self
            .variables
            .iter()
            .filter(|v| {
                let type_condition = allowed_types.iter().any(|allowed_type| {
                    var_type::way_to_type(random, &v.var_type(), &allowed_type, &mut false)
                        .is_some()
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

    /// Return a randomly chosen lambda function that has a return type matching one of the list of types given as parameter
    /// Return None if there is none
    pub fn get_random_lambda(
        &self,
        random: &mut Random,
        allowed_types: Vec<VarType>,
    ) -> Option<&Lambda> {
        let filtered_lambdas: Vec<&Lambda> = self
            .lambdas
            .iter()
            .filter(|l| match l.ret_type() {
                Some(t) => allowed_types.iter().any(|allowed_type| {
                    var_type::way_to_type(random, t, &allowed_type, &mut false).is_some()
                }),
                None => false,
            })
            .collect();

        if filtered_lambdas.len() == 0 {
            return None;
        }

        Some(random.choose_random_item_from_vec(&filtered_lambdas))
    }

    pub fn next_variable_name(&self) -> String {
        format!("var{}", self.variables.len() + self.name_number_to_skip)
    }

    pub fn next_lambda_name(&self) -> String {
        format!("lambda{}", self.lambdas.len() + self.name_number_to_skip)
    }
}
