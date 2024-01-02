use crate::variables::bloc_variables::BlocVariables;
use crate::variables::variable::Variable;
use crate::variables::var_type;
use crate::random;
use crate::instructions::generate_instruction::fill_bloc_variables;

pub fn generate_operation_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    let mut variables_used: Vec<Variable> = Vec::new();

    let chosen_type = var_type::random_type();

    instruction = format!("{}{}", instruction, fill_bloc_variables(bloc_variables, &mut variables_used, [chosen_type.clone()].to_vec()));
    match bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), Some(true)) {
        Some(assigned_var) => instruction = format!("{}{} = {}", instruction, assigned_var.name(), variables_used[0].name()),
        None => instruction = format!("{}{}", instruction, variables_used[0].name()),
    }

    for var in variables_used.iter().skip(1) {
        instruction = format!("{} {} {}", instruction, random::choose_random_item_from_vec(&var_type::supported_operations_by_type(chosen_type.clone())), var.name());
    }

    instruction + ";\n"
}