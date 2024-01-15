use crate::instructions::comparison_instruction::generate_comparison_instruction;
use crate::variables::bloc_variables::BlocVariables;

pub fn generate_assert_instruction(bloc_variables: &mut BlocVariables) -> String {

    format!("assert({})\n", generate_comparison_instruction(bloc_variables))
}