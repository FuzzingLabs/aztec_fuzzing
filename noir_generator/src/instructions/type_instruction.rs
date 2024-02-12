use crate::constants::MAX_OPERATION_DEPTH;
use crate::functions::list_functions::ListFunctions;
use crate::variables::bloc_variables::BlocVariables;
use crate::variables::value;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random;

fn type_operation_rec(bloc_variables: &BlocVariables, chosen_type: &VarType, depth: usize) -> Operation {

    let element1: Operand = if depth ==  0 || random::gen_bool() {
        if random::gen_bool() {
            let var = bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(bloc_variables, chosen_type, depth - 1)))
    };

    let element2: Operand = if depth ==  0 || random::gen_bool() {
        if random::gen_bool() {
            let var = bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(bloc_variables, chosen_type, depth - 1)))
    };

    Operation::new(
        chosen_type,
        None,
        element1,
        element2,
    )
}

pub fn generate_type_instruction(bloc_variables: &BlocVariables, list_functions: &ListFunctions, instruction_type: &VarType) -> String {

    match random::gen_range(0, 3) {
        0 => value::random_value(instruction_type).to_string(),
        1 => {
            if bloc_variables.is_empty() | var_type::supported_arithmetic_operator_by_type(instruction_type).is_empty() {
                return value::random_value(instruction_type).to_string();
            }
            type_operation_rec(bloc_variables, instruction_type, MAX_OPERATION_DEPTH).to_string()
        },
        2 => {
            if list_functions.is_empty() {
                return value::random_value(instruction_type).to_string();
            }
            match list_functions.call_by_type(instruction_type){
                Some(s) => s,
                None => return value::random_value(instruction_type).to_string(),
            }
        }
        _ => "".to_string()
    }
}