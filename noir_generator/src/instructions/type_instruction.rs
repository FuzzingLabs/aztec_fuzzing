use crate::constants::MAX_OPERATION_DEPTH;
use crate::functions::list_functions::ListFunctions;
use crate::variables::bloc_data::BlocData;
use crate::variables::list_structs::ListStructs;
use crate::variables::value;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::random::Random;

fn type_operation_rec(random: &mut Random, bloc_variables: &BlocData, chosen_type: &VarType, depth: usize) -> Operation {

    let element1: Operand = if depth ==  0 || random.gen_bool() {
        if random.gen_bool() {
            let random_bool = random.gen_bool();
            let var = bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), random_bool);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(random, chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(random, chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(random, bloc_variables, chosen_type, depth - 1)))
    };

    let element2: Operand = if depth ==  0 || random.gen_bool() {
        if random.gen_bool() {
            let random_bool = random.gen_bool();
            let var = bloc_variables.get_random_variable(random, [chosen_type.clone()].to_vec(), random_bool);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(random, chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(random, chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(random, bloc_variables, chosen_type, depth - 1)))
    };

    Operation::new(
        chosen_type,
        random.choose_random_item_from_vec(&var_type::supported_arithmetic_operator_by_type(chosen_type)),
        element1,
        element2,
    )
}

pub fn generate_type_instruction(random: &mut Random, bloc_variables: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs, instruction_type: &VarType) -> String {

    match random.gen_range(0, 4) {
        0 => {
            match bloc_variables.get_random_variable(random, vec!(instruction_type.clone()), false){
                Some(v) => return v.name_and_way(random, instruction_type),
                None => return value::random_value(random, instruction_type).to_string(),
            }
        },
        1 => {
            if bloc_variables.is_variables_empty() | var_type::supported_arithmetic_operator_by_type(instruction_type).is_empty() {
                return value::random_value(random, instruction_type).to_string();
            }
            type_operation_rec(random, bloc_variables, instruction_type, MAX_OPERATION_DEPTH).to_string(random)
        },
        2 => {
            if list_functions.is_empty() {
                return value::random_value(random, instruction_type).to_string();
            }
            match list_functions.call_by_type(random, bloc_variables, list_structs, instruction_type){
                Some(s) => s,
                None => return value::random_value(random, instruction_type).to_string(),
            }
        }
        3 => {
            if bloc_variables.is_lambdas_empty() {
                return value::random_value(random, instruction_type).to_string();
            }
            match bloc_variables.get_random_lambda(random, vec!(instruction_type.clone())){
                Some(l) => format!("{}{}", l.call(random), var_type::way_to_type(random, 
                    &l.ret_type().clone().expect("lambda with no ret type"), instruction_type).expect("no way to type")),
                None => return value::random_value(random, instruction_type).to_string(),
            }
        }
        _ => "".to_string()
    }
}