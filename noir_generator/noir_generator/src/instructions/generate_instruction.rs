use crate::variables::bloc_variables::BlocVariables;
use crate::variables::variable::Variable;
use crate::random;
use crate::instructions::generate_assert;
use crate::instructions::generate_operation;

pub fn fill_bloc_variables(bloc_variables: &mut BlocVariables, variables_used: &mut Vec<Variable>, allowed_types: Vec<&'static str>) -> String {
    let mut variables_to_initialize: String = String::new();
    let n = random::generate_random_number(2, 100);
    for _ in 0..n {
        match bloc_variables.get_random_variable(variables_used.clone(), allowed_types.clone(), None) {
            Some(variable) => {
                variables_used.push(variable.clone());
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

    match random::generate_random_number(0, 2) {
        0 => instruction = format!("{}{}", instruction, generate_operation::generate_operation_instruction(bloc_variables)),
        1 => instruction = format!("{}{}", instruction, generate_assert::generate_assert_instruction(bloc_variables)),
        _ => {}
    };

    instruction
}