use crate::instructions::comparison_instruction::generate_comparison_instruction;
use crate::random::Random;
use crate::variables::bloc_variables::BlocVariables;

pub fn generate_assert_instruction(random: &mut Random, bloc_variables: &BlocVariables) -> String {
    format!("assert({});\n", generate_comparison_instruction(random, bloc_variables))
}