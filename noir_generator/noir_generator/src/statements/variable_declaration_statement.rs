use crate::variables::bloc_variables::BlocVariables;
use crate::variables::var_type;
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_variable_declaration(bloc_variables: &mut BlocVariables) -> String {

    let chosen_type = var_type::random_type();

    let variables_used = bloc_variables.get_variables_by_types([chosen_type.clone()].to_vec());

    let new_variable = bloc_variables.new_variable([chosen_type.clone()].to_vec(), None);
    format!("{} = {};\n", new_variable.initialise(), generate_type_instruction(&variables_used, chosen_type))
}