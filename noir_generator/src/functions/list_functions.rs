use crate::{constants::CONFIG, random::Random, variables::{basic_trait::BasicTrait, bloc_data::BlocData, list_structs::ListStructs, var_type::{self, is_same_type, VarType}, variable::Variable}};
use super::function::Function;

// Represent the list of every function in the generated code
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

    // Return a string representing the call of a function that has a return type identical to the type given as a parameter
    // Return None if there is no function with this return type
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

    // Add a new randomly generated function to this list
    pub fn add_function(&mut self, random: &mut Random, list_global: &BlocData, list_structs: &ListStructs, is_main: bool) -> String {
        let mut bloc_variables = BlocData::new();
        let function;

        if is_main{
            for _ in 0..CONFIG.max_function_arguments{
                bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &var_type::random_basic_type(random)));
            }
            function = Function::new("main".to_string(), false, bloc_variables, None);
        
        } else {
            let mut use_generic = false;
            let mut vec_trait = Vec::new();

            for t in BasicTrait::iterator() {
                if random.gen_bool() {
                    vec_trait.push(t);
                }
            }

            for _ in 0..CONFIG.max_function_arguments{
                let var_type = var_type::random_type(random, list_structs);
                let rand = random.gen_range(0, 10);
                if rand == 0 {
                    let lambda = bloc_variables.create_lambda(random, list_structs, &var_type);
                    bloc_variables.add_lambda(lambda);
                } else if rand == 1 {
                    use_generic = true;
                    bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &VarType::generic(vec_trait.clone())));
                } else if rand == 2 {
                    use_generic = true;
                    bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &VarType::array(Box::new(VarType::generic(vec_trait.clone())), usize::max_value())));
                }
                else {
                    bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &var_type));
                }
            }

            let rand = random.gen_range(0, 10);
            let ret_type = if rand == 0 {
                None
            } else {
                Some(var_type::random_type(random, list_structs))
            };

            function = Function::new(format!("func{}", self.next_id()), random.gen_bool(), bloc_variables, ret_type);
        }

        let ret = function.generate_function_code(random, list_global, self, list_structs);
        self.functions.push(function);
        
        ret
    }

    fn next_id(&self) -> usize {
        self.functions.len()+1
    }
}