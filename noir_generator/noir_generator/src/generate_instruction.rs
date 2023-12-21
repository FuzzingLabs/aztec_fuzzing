use crate::bloc_variables::BlocVariables;
use crate::variable::Variable;
use crate::types;
use crate::random;

fn fill_bloc_variables(bloc_variables: &mut BlocVariables, variables_used: &mut Vec<Variable>, allowed_types: Vec<&'static str>) -> String {
    let mut variables_to_initialize: String = String::new();
    let n = random::generate_random_number(2, 10);
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

pub fn generate_operation_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    let mut variables_used: Vec<Variable> = Vec::new();

    let chosen_type = random::select_random_str_from_vec(types::types());

    instruction = format!("{}{}", instruction, fill_bloc_variables(bloc_variables, &mut variables_used, [chosen_type].to_vec()));

    instruction = format!("{}{}", instruction, variables_used[0].name());

    // Utilisez toutes les variables dans la génération d'instruction
    for var in variables_used.iter().skip(1) {
        instruction = format!("{} {} {}", instruction, random::select_random_str_from_vec(types::supported_operations_for_type(chosen_type)), var.name());
    }

    instruction + ";\n"
}

pub fn generate_random_instruction(bloc_variables: &mut BlocVariables) -> String {
    generate_operation_instruction(bloc_variables)
}