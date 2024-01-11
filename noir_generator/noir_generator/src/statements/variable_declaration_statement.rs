use crate::variables::bloc_variables::BlocVariables;
use crate::variables::var_type;
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_variable_declaration(bloc_variables: &mut BlocVariables) -> String {

    let chosen_type = var_type::random_type();

    let instr_string = generate_type_instruction(bloc_variables, chosen_type.clone());

    let new_variable = bloc_variables.new_variable([chosen_type.clone()].to_vec(), None);
    format!("{} = {};\n", new_variable.initialise(), instr_string)
}