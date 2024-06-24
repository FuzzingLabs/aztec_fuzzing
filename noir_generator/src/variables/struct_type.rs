use crate::{constants::CONFIG, functions::{list_functions::ListFunctions, method::Method}, random::Random, variables::{var_type, variable::Variable}};

use super::{bloc_data::BlocData, list_structs::ListStructs, var_type::{is_same_type, VarType}};

// Represent a structure by its name, the list of the types of its keys, and the list of its methods
#[derive(Clone)]
pub struct StructType{
    key_types: Vec<(VarType, String)>,
    name: String,
    methods: Vec<Method>
}

impl StructType {
    pub fn new(key_types: Vec<(VarType, String)>, name: String) -> Self {
        Self {
            key_types,
            name,
            methods: Vec::new(),
        }
    }

    pub fn key_types(&self) -> &Vec<(VarType, String)> {
        &self.key_types
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    // Return a string representing the complete code required for this structure
    pub fn generate_struct_code(&mut self, random: &mut Random, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
        let mut struct_string = format!("struct {} {{\n", self.name);
        for (_, key_type) in self.key_types.iter().enumerate() {
            struct_string = format!("{}{}: {},\n", struct_string, key_type.1, key_type.0);
        }
        struct_string = format!("{}}}\n\n", struct_string);

        format!("{}{}", struct_string, self.generate_impl_code(random, list_global, list_functions, list_structs))
    }

    // Return a string representing the code required for the implementation part of this structure
    pub fn generate_impl_code(&mut self, random: &mut Random, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {
        let mut impl_string = format!("impl {} {{\n", self.name);
        for i in 0..random.gen_range(0, CONFIG.max_method_by_struct) {
            let mut bloc_variables = BlocData::new();
            for _ in 0..random.gen_range(0, CONFIG.max_method_arguments){
                bloc_variables.add_variable(Variable::new(bloc_variables.next_variable_name(), false, &var_type::random_type(random, list_structs)));
            }
            let new_method = Method::new(format!("method{}", i+1), bloc_variables, 
            Some(var_type::random_type(random, list_structs)), self.name().clone(), random.gen_bool());

            impl_string = format!("{}\n{}\n", impl_string, new_method.initialize(random, list_global,  list_functions, list_structs, self));

            self.methods.push(new_method)
        }
        format!("{}}}\n\n", impl_string)
    }



    // Return a string representing the code required to call a method of this structure that has the same return type as the type given in the parameter
    // Return None if it's not possible
    pub fn call_by_type(&self, random: &mut Random, bloc_variables: &BlocData, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs, ret_type: &VarType, depth: usize, self_var: Option<String>) -> Option<String> {
        let valid_method: Vec<&Method> = self.methods.iter().filter(|&e| {
            if let Some(e_ret_type) = e.ret_type() {
                is_same_type(e_ret_type, ret_type)
            } else {
                false
            }
        }).collect();
        
        if valid_method.is_empty() {
            return None;
        }
        
        Some(random.choose_random_item_from_vec(&valid_method).call(random, bloc_variables, list_global, list_functions, list_structs, depth, self_var))
    }
}