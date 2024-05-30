use crate::constants::CONFIG;
use crate::variables::bloc_data::BlocData;
use crate::variables::value;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random::Random;

#[derive(Clone)]
pub enum Trait {
    Eq,
    Ord,
}

pub fn generate_generic_instruction(random: &mut Random, bloc_variables: &BlocData, depth: usize) -> String {
    let element1 = bloc_variables.get_random_variable(random, vec!(&VarType::generic(Vec::new())), false).expect("No generic variable in generic instruction");
    let element2 = bloc_variables.get_random_variable(random, vec!(&VarType::generic(Vec::new())), false).expect("No generic variable in generic instruction");

    Operation::new(
        element1.var_type(),
        random.choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(element1.var_type())),
        Operand::Variable(element1.clone()),
        Operand::Variable(element2.clone()),
    ).to_string(random)
}