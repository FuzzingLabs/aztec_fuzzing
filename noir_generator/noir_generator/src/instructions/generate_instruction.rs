use crate::variables::bloc_variables::BlocVariables;
use crate::variables::variable::Variable;
use crate::random;
use crate::instructions::generate_variable_declaration;
use crate::instructions::generate_assert;
use crate::instructions::generate_operation;


const NB_MAX_VARIABLE: u32 = 10;

pub fn fill_bloc_variables(bloc_variables: &mut BlocVariables, variables_used: &mut Vec<Variable>, allowed_types: Vec<&'static str>) -> String {
    let mut variables_to_initialize: String = String::new();

    for _ in 0..random::generate_random_number(2, NB_MAX_VARIABLE) {
        match bloc_variables.get_random_variable(allowed_types.clone(), None) {
            Some(variable) => {
                if variables_used.contains(variable) && random::generate_random_bool() {
                    let new_variable = bloc_variables.new_variable(allowed_types.clone(), None);
                    variables_to_initialize = format!("{}{}", variables_to_initialize, new_variable.initialise());
                    variables_used.push(new_variable.clone());
                } else {
                    variables_used.push(variable.clone());
                }
            }
            None => {
                let new_variable = bloc_variables.new_variable(allowed_types.clone(), None);
                variables_to_initialize = format!("{}{}", variables_to_initialize, new_variable.initialise());
                variables_used.push(new_variable.clone());
            }
        }
    }
    variables_to_initialize
}

pub fn generate_random_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();

    match random::generate_random_number(0, 3) {
        0 => instruction = format!("{}{}", instruction, generate_operation::generate_operation_instruction(bloc_variables)),
        1 => instruction = format!("{}{}", instruction, generate_assert::generate_assert_instruction(bloc_variables)),
        2 => instruction = format!("{}{}", instruction, generate_variable_declaration::generate_variable_declaration(bloc_variables)),
        _ => {}
    };

    instruction
}