use crate::variables::bloc_variables::BlocVariables;
use crate::variables::variable::Variable;
use crate::variables::var_type::{self, VarType};
use crate::variables::interaction::Interaction;
use crate::variables::value_provider::ValueProviderEnum;
use crate::random;
use crate::instructions::generate_instruction::fill_bloc_variables;


const MAX_INTERACTIONS_DEPTH: usize = 10;

fn generate_random_interaction(variables: &Vec<Variable>, chosen_type:VarType, depth: usize) -> Interaction {

    let element1: ValueProviderEnum = if true || depth ==  0 || random::gen_bool() {
        // Choisir une variable aléatoire
        ValueProviderEnum::Variable(random::choose_random_item_from_vec(&variables))
    } else {
        // Choisir une interaction récursive aléatoire
        ValueProviderEnum::Interaction(Box::new(generate_random_interaction(&variables, chosen_type.clone(), depth - 1)))
    };

    let element2: ValueProviderEnum = if true || depth ==  0 || random::gen_bool() {
        // Choisir une variable aléatoire
        ValueProviderEnum::Variable(random::choose_random_item_from_vec(&variables))
    } else {
        // Choisir une interaction récursive aléatoire
        ValueProviderEnum::Interaction(Box::new(generate_random_interaction(&variables, chosen_type, depth - 1)))
    };

    Interaction::new(
        None,
        element1,
        element2,
    )
}

pub fn generate_operation_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    let mut variables_used: Vec<Variable> = Vec::new();

    let chosen_type = var_type::random_basic_type();

    instruction = format!("{}{}", instruction, fill_bloc_variables(bloc_variables, &mut variables_used, [chosen_type.clone()].to_vec()));

    match bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), Some(true)) {
        Some(assigned_var) => instruction = format!("{}{} = ", instruction, assigned_var.name()),
        None => {
            let assigned_var = bloc_variables.new_variable([chosen_type.clone()].to_vec(), Some(true));
            instruction = format!("{}{}", instruction, assigned_var.initialise());
            instruction = format!("{}{} = ", instruction, assigned_var.name());
            variables_used.push(assigned_var);
        },
    }

    instruction = format!("{}{};\n", instruction, generate_random_interaction(&variables_used, chosen_type, random::gen_range(1, MAX_INTERACTIONS_DEPTH)));
    instruction

}