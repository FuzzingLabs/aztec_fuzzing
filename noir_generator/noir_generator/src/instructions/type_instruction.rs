use crate::constants::MAX_OPERATION_DEPTH;
use crate::variables::bloc_variables::BlocVariables;
use crate::variables::value;
use crate::variables::variable::Variable;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random;

fn type_operation_rec(bloc_variables: &mut BlocVariables, chosen_type:VarType, depth: usize) -> Operation {

    let element1: Operand = if depth ==  0 || random::gen_bool() {
        if random::gen_bool() {
            let var = bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(&chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(&chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(bloc_variables, chosen_type.clone(), depth - 1)))
    };

    let element2: Operand = if depth ==  0 || random::gen_bool() {
        if random::gen_bool() {
            let var = bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(&chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(&chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(bloc_variables, chosen_type.clone(), depth - 1)))
    };

    Operation::new(
        chosen_type,
        None,
        element1,
        element2,
    )
}

pub fn generate_type_instruction(bloc_variables: &mut BlocVariables, instruction_type: VarType) -> String {
    if bloc_variables.is_empty() | var_type::supported_arithmetic_operator_by_type(instruction_type.clone()).is_empty() {
        return value::random_value(&instruction_type).to_string();
    }

    match random::gen_range(0, 1) {
        0 => value::random_value(&instruction_type).to_string(),
        1 => type_operation_rec(bloc_variables, instruction_type, MAX_OPERATION_DEPTH).to_string(),
        _ => "".to_string()
    }
}