use rand::distributions::uniform::SampleRange;

use crate::variables::bloc_variables::BlocVariables;
use crate::variables::value;
use crate::variables::variable::Variable;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random;


const MAX_OPERATION_DEPTH: usize = 10;

fn operation_rec(variables: &Vec<Variable>, chosen_type:VarType, depth: usize) -> Operation {

    let element1: Operand = if depth ==  0 || random::gen_bool() {
        if random::gen_bool() {
            Operand::Variable(random::choose_random_item_from_vec(variables))
        } else {
            Operand::Value(value::random_value(&chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(operation_rec(&variables, chosen_type.clone(), depth - 1)))
    };

    let element2: Operand = if depth ==  0 || random::gen_bool() {
        if random::gen_bool() {
            Operand::Variable(random::choose_random_item_from_vec(variables))
        } else {
            Operand::Value(value::random_value(&chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(operation_rec(&variables, chosen_type, depth - 1)))
    };

    Operation::new(
        None,
        element1,
        element2,
    )
}

pub fn generate_type_instruction(variables: &Vec<Variable>, instruction_type: VarType) -> String {
    if variables.is_empty() | var_type::supported_arithmetic_operator_by_type(instruction_type.clone()).is_empty() {
        return value::random_value(&instruction_type).to_string();
    }

    match random::gen_range(0, 1) {
        0 => value::random_value(&instruction_type).to_string(),
        1 => operation_rec(&variables, instruction_type, MAX_OPERATION_DEPTH).to_string(),
        _ => "".to_string()
    }
}