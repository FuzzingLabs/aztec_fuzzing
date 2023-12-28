use crate::variables::bloc_variables::BlocVariables;
use crate::variables::variable::Variable;
use crate::variables::types;
use crate::random;
use crate::instructions::generate_instruction::fill_bloc_variables;

pub fn generate_operation_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    let mut variables_used: Vec<Variable> = Vec::new();

    let chosen_type = random::select_random_str_from_vec(types::basic_types());

    instruction = format!("{}{}", instruction, fill_bloc_variables(bloc_variables, &mut variables_used, [chosen_type].to_vec()));

    instruction = format!("{}{} = {}", instruction, variables_used[0].name(), variables_used[1].name());

    // Utilisez toutes les variables dans la génération d'instruction
    for var in variables_used.iter().skip(2) {
        instruction = format!("{} {} {}", instruction, random::select_random_str_from_vec(types::supported_operations_by_type(chosen_type)), var.name());
    }

    instruction + "\n"
}