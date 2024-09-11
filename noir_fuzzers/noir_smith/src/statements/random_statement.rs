use super::assert_statement;
use super::declaration_statement;
use super::operation_statement;
use crate::functions::list_functions::ListFunctions;
use crate::tools::random::Random;
use crate::variables::bloc_data::BlocData;
use crate::variables::list_structs::ListStructs;

/// Return a string that represents the code of a randomly chosen statement

pub fn generate_random_statement(
    random: &mut Random,
    bloc_variables: &mut BlocData,
    list_global: &BlocData,
    list_functions: &ListFunctions,
    list_structs: &ListStructs,
) -> String {
    // To reduce the chance to generate a lambda function
    if random.gen_range(0, 10) == 0 {
        return declaration_statement::generate_lambda_declaration(
            random,
            bloc_variables,
            list_global,
            list_functions,
            list_structs,
        );
    }

    match random.gen_range(0, 2) {
        0 => declaration_statement::generate_variable_declaration(
            random,
            bloc_variables,
            list_global,
            list_functions,
            list_structs,
        ),
        1 => operation_statement::generate_operation_instruction(
            random,
            bloc_variables,
            list_global,
            list_functions,
            list_structs,
        ),
        2 => assert_statement::generate_assert_instruction(random, bloc_variables),
        _ => String::new(),
    }
}
