use crate::instructions::comparison_instruction::generate_comparison_instruction;
use crate::random::Random;
use crate::variables::bloc_data::BlocData;

pub fn generate_assert_instruction(random: &mut Random, bloc_variables: &BlocData) -> String {
    format!("assert({});\n", generate_comparison_instruction(random, bloc_variables))
}