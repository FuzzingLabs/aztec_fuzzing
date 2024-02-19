use rand::Rng;

use crate::constants::MAX_OPERATION_DEPTH;
use crate::variables::bloc_variables::BlocVariables;
use crate::variables::value;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random::Random;

fn get_leaf(random: &mut Random, bloc_variables: &BlocVariables) -> Option<Operation> {
    let random_bool = random.gen_bool();
    let var = match bloc_variables.get_random_variable(random, var_type::basic_types(), random_bool){
        Some(v) => v,
        None => return None,
    };

    let chosen_type = {
        let mut vec_type = Vec::new();
        for var_type in var_type::basic_types() {
            if var_type::way_to_type(random, var.var_type(), &var_type).is_some() {
                vec_type.push(var_type);
            }
        }

        random.choose_random_item_from_vec(&vec_type)
    };

    let elem1 = Operand::Variable(var.clone());

    let elem2 = if random.gen_bool() {
        let random_bool = random.gen_bool();
        match bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), random_bool){
            Some(v) => Operand::Variable(v.clone()),
            //Should never happen
            None => return None,
        }
    } else {
        Operand::Value(value::random_value(random, &chosen_type), chosen_type.clone())
    };

    Some(Operation::new(
        &chosen_type,
        random.choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(&chosen_type)),
        elem1,
        elem2,
    ))
}

fn comparison_rec(random: &mut Random, bloc_variables: &BlocVariables, depth: usize) -> Operation {

    let element1 = if depth ==  0 || random.gen_bool() {
        match get_leaf(random, bloc_variables) {
            Some(v) => Operand::Operation(Box::new(v)),
            None => Operand::Value(value::random_value(random, &VarType::bool), VarType::bool),
        }
    } else {
        Operand::Operation(Box::new(comparison_rec(random, bloc_variables, depth - 1)))
    };

    let element2 = if depth ==  0 || random.gen_bool() {
        match get_leaf(random, bloc_variables) {
            Some(v) => Operand::Operation(Box::new(v)),
            None => Operand::Value(value::random_value(random, &VarType::bool), VarType::bool),
        }
    } else {
        Operand::Operation(Box::new(comparison_rec(random, bloc_variables, depth - 1)))
    };

    Operation::new(
        &VarType::bool,
        random.choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(&VarType::bool)),
        element1,
        element2,
    )
}

pub fn generate_comparison_instruction(random: &mut Random, bloc_variables: &BlocVariables) -> String {
    if bloc_variables.is_empty() {
        return value::random_value(random, &VarType::bool).to_string();
    }

    comparison_rec(random, bloc_variables, MAX_OPERATION_DEPTH).to_string(random)
}