use crate::functions::list_functions::ListFunctions;
use crate::variables::bloc_variables::BlocVariables;
use crate::variables::var_type;
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_variable_declaration(bloc_variables: &mut BlocVariables, list_functions: &ListFunctions) -> String {

    let chosen_type = var_type::random_type();

    let instr_string = generate_type_instruction(bloc_variables, list_functions, &chosen_type);

    let new_variable = bloc_variables.new_variable(&chosen_type, None);
    format!("{} = {};\n", new_variable.initialize(), instr_string)
}