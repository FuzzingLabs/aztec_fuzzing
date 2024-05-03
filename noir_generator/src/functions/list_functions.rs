use crate::{constants::CONFIG, random::Random, variables::{bloc_data::BlocData, list_structs::ListStructs, var_type::{self, is_same_type, VarType}, variable::Variable}};
use super::function::Function;

#[derive(Clone)]
pub struct ListFunctions{
    functions: Vec<Function>
}

impl ListFunctions {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    pub fn call_by_type(&self, random: &mut Random, bloc_variables: &BlocData, list_global: &BlocData, list_structs: &ListStructs, ret_type: &VarType, depth: usize) -> Option<String> {
        let valid_function: Vec<&Function> = self.functions.iter().filter(|&e| {
            if let Some(e_ret_type) = e.ret_type() {
                is_same_type(e_ret_type, ret_type)
            } else {
                false
            }
        }).collect();
        
        if valid_function.is_empty() {
            return None;
        }
        
        Some(random.choose_random_item_from_vec(&valid_function).call(random, bloc_variables, list_global, self, list_structs, depth))
    }

    pub fn add_function(&mut self, random: &mut Random, list_global: &BlocData, list_structs: &ListStructs, is_main: bool) -> String {
        let mut bloc_variables = BlocData::new();
        let function;
        if is_main{
            for _ in 0..CONFIG.max_function_arguments{
                bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &var_type::random_basic_type(random)));
            }
            function = Function::new("main".to_string(), false, bloc_variables, None);
        } else {
            for _ in 0..CONFIG.max_function_arguments{
                let var_type = var_type::random_type(random, list_structs);
                if random.gen_range(0, 10) == 0 {
                    let lambda = bloc_variables.create_lambda(random, list_structs, &var_type);
                    bloc_variables.add_lambda(lambda);
                } else {
                    bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &var_type));
                }
            }
            function = Function::new(format!("func{}", self.next_id()), random.gen_bool(), bloc_variables, Some(var_type::random_type(random, list_structs)));
        }

        let ret = function.generate_function_code(random, list_global, self, list_structs);
        self.functions.push(function);
        
        ret
    }

    fn next_id(&self) -> usize {
        self.functions.len()+1
    }
}