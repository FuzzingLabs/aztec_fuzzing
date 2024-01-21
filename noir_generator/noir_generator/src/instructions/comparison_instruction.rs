use crate::constants::MAX_OPERATION_DEPTH;
use crate::variables::bloc_variables::BlocVariables;
use crate::variables::value;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random;

fn get_leaf(bloc_variables: &mut BlocVariables) -> Operation {
    let var = match bloc_variables.get_random_variable(var_type::basic_types(), None){
        Some(v) => v,
        None => {
            let chosen_type = var_type::random_basic_type();
            return Operation::new(
                &chosen_type,
                Some(random::choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(&chosen_type))),
                Operand::Value(value::random_value(&chosen_type), chosen_type.clone()),
                Operand::Value(value::random_value(&chosen_type), chosen_type.clone()),
            );
        },
    };

    let chosen_type = {
        let mut vec_type = Vec::new();
        for var_type in var_type::basic_types() {
            if var_type::way_to_type(var.var_type(), &var_type).is_some() {
                vec_type.push(var_type);
            }
        }

        &random::choose_random_item_from_vec(&vec_type)
    };

    let elem1 = Operand::Variable(var.clone());

    let elem2 = if random::gen_bool() {
        match bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), None){
            Some(v) => Operand::Variable(v.clone()),
            //Should never happen
            None => panic!("get_leaf() with no avaible variable for elem2"),
        }
    } else {
        Operand::Value(value::random_value(chosen_type), chosen_type.clone())
    };

    Operation::new(
        chosen_type,
        Some(random::choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(chosen_type))),
        elem1,
        elem2,
    )
}

fn comparison_rec(bloc_variables: &mut BlocVariables, depth: usize) -> Operation {

    let element1 = if depth ==  0 || random::gen_bool() {
        Operand::Operation(Box::new(get_leaf(bloc_variables)))
    } else {
        Operand::Operation(Box::new(comparison_rec(bloc_variables, depth - 1)))
    };

    let element2 = if depth ==  0 || random::gen_bool() {
        Operand::Operation(Box::new(get_leaf(bloc_variables)))
    } else {
        Operand::Operation(Box::new(comparison_rec(bloc_variables, depth - 1)))
    };

    Operation::new(
        &VarType::bool,
        Some(random::choose_random_item_from_vec(&var_type::supported_comparator_operator_by_type(&VarType::bool))),
        element1,
        element2,
    )
}

pub fn generate_comparison_instruction(bloc_variables: &mut BlocVariables) -> String {
    if bloc_variables.is_empty() {
        return value::random_value(&VarType::bool).to_string();
    }

    comparison_rec(bloc_variables, MAX_OPERATION_DEPTH).to_string()
}