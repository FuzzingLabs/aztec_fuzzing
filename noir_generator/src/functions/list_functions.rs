use std::fmt::format;

use crate::{constants::{self, NB_MAX_INSTRUCTION_BY_FUNCTION}, functions::{function, list_functions}, instructions::comparison_instruction::generate_comparison_instruction, random, statements::random_statement, variables::{bloc_variables::{self, BlocVariables}, operand::Operand, value, var_type::{self, VarType}}};
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

    pub fn call_by_type(&self, ret_type: &VarType) -> Option<String> {
        let valid_function: Vec<&Function> = self.functions.iter().filter(|&e| {
            if let Some(e_ret_type) = e.ret_type() {
                *e_ret_type == *ret_type
            } else {
                false
            }
        }).collect();
        
        if valid_function.is_empty() {
            return None;
        }
        
        Some(random::choose_random_item_from_vec(&valid_function).call())
    }

    pub fn add_function(&mut self, is_main: bool) {
        let mut bloc_variables = BlocVariables::new();

        let mut function;
        
        if is_main{
            for _ in 0..constants::NB_MAX_ARGUMENTS{
                bloc_variables.new_variable(&var_type::random_basic_type(), Some(false));
            }
            function = Function::new("main".to_string(), Some(false), bloc_variables, None);
        } else {
            for _ in 0..constants::NB_MAX_ARGUMENTS{
                bloc_variables.new_variable(&var_type::random_type(), Some(false));
            }
            function = Function::new(random::gen_name(), None, bloc_variables, Some(var_type::random_type()));
        }
        self.functions.push(function);
    }

    pub fn generate_all_functions_core(&self) -> String {
        let mut ret = String::new();
        for i in &self.functions {
            ret = format!("{}{}", ret, i.generate_core(self));
        }
        ret
    }
    
}