use crate::functions::list_functions::{self, ListFunctions};
use crate::random::Random;
use crate::variables::{bloc_variables::BlocVariables, var_type};
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_operation_instruction(random: &mut Random, bloc_variables: &mut BlocVariables, list_functions: &ListFunctions) -> String {
    let mut instruction: String = String::new();
    
    let chosen_type = var_type::random_basic_type(random);

    let instr_string = generate_type_instruction(random, bloc_variables, list_functions, &chosen_type);

    match bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), true) {
        Some(assigned_var) => instruction = format!("{}{} = ", instruction, assigned_var.name_and_way(random, &chosen_type)),
        None => {
            let assigned_var = bloc_variables.new_variable(&chosen_type, random.gen_bool());
            instruction = format!("{}{} = ", instruction, assigned_var.initialize());
        },
    }

    format!("{}{};\n", instruction, instr_string)

}