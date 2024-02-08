use crate::functions::list_functions;
use crate::functions::list_functions::ListFunctions;
use crate::variables::bloc_variables::BlocVariables;
use crate::random;
use super::variable_declaration_statement;
use super::assert_statement;
use super::operation_statement;


pub fn generate_random_statement(bloc_variables: &mut BlocVariables, list_functions: &ListFunctions) -> String {
    let mut instruction: String = String::new();

    match random::gen_range(0, 3) {
        0 => instruction = format!("{}{}", instruction, variable_declaration_statement::generate_variable_declaration(bloc_variables, list_functions)),
        1 => instruction = format!("{}{}", instruction, operation_statement::generate_operation_instruction(bloc_variables, list_functions)),
        2 => instruction = format!("{}{}", instruction, assert_statement::generate_assert_instruction(bloc_variables)),
        _ => {}
    };

    instruction
}