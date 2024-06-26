use crate::tools::constants::CONFIG;
use crate::functions::list_functions::ListFunctions;
use crate::variables::bloc_data::BlocData;
use crate::variables::list_structs::ListStructs;
use crate::variables::value;
use crate::variables::var_type::{self, VarType};
use crate::variables::operation::Operation;
use crate::variables::operand::Operand;
use crate::tools::random::Random;

/// Function to generate nested instructions with recursive calls. The max depth is specified in config
fn type_operation_rec(random: &mut Random, bloc_data: &BlocData, chosen_type: &VarType, depth: usize) -> Operation {

    let element1: Operand = if depth ==  0 || random.gen_bool() {
        if random.gen_bool() {
            let random_bool = random.gen_bool();
            let var = bloc_data.get_random_variable(random, vec![&chosen_type], random_bool);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(random, chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(random, chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(random, bloc_data, chosen_type, depth - 1)))
    };

    let element2: Operand = if depth ==  0 || random.gen_bool() {
        if random.gen_bool() {
            let random_bool = random.gen_bool();
            let var = bloc_data.get_random_variable(random, vec![&chosen_type], random_bool);
            match var {
                Some(v) => Operand::Variable(v.clone()),
                None => Operand::Value(value::random_value(random, chosen_type), chosen_type.clone()),
            }
        } else {
            Operand::Value(value::random_value(random, chosen_type), chosen_type.clone())
        }
    } else {
        Operand::Operation(Box::new(type_operation_rec(random, bloc_data, chosen_type, depth - 1)))
    };

    Operation::new(
        chosen_type,
        random.choose_random_item_from_vec(&var_type::supported_arithmetic_operator_by_type(chosen_type)),
        element1,
        element2,
    )
}

/// Return a string that represents a randomly generated instruction of the type given in the parameter
/// List all possible instructions, each of which can be nested:
///     Raw value
///     Variable
///     Function call
///     Structure method call
///     Lambda function call
pub fn generate_type_instruction(random: &mut Random, bloc_data: &BlocData, list_global: &BlocData, list_functions: &ListFunctions, list_structs: &ListStructs, instruction_type: &VarType, depth: usize) -> String {
    if depth == 0 {
        match random.gen_range(0, 2) {
            0 => {
                match bloc_data.get_random_variable(random, vec!(&instruction_type), matches!(instruction_type, VarType::reference(_))){
                    Some(v) => return v.name_and_way(random, instruction_type),
                    None => return value::random_value(random, instruction_type).to_string(),
                }
            },
            1 => return value::random_value(random, instruction_type).to_string(),
            _ => "".to_string()
        }
    } else {
        match random.gen_range(0, 5) {
            0 => {
                match bloc_data.get_random_variable(random, vec!(&instruction_type), matches!(instruction_type, VarType::reference(_))){
                    Some(v) => return v.name_and_way(random, instruction_type),
                    None => return value::random_value(random, instruction_type).to_string(),
                }
            },
            1 => {
                if bloc_data.is_variables_empty() | var_type::supported_arithmetic_operator_by_type(instruction_type).is_empty() {
                    return value::random_value(random, instruction_type).to_string();
                }
                type_operation_rec(random, bloc_data, instruction_type, CONFIG.max_operation_depth).to_string(random)
            },
            2 => {
                if list_functions.is_empty() {
                    return value::random_value(random, instruction_type).to_string();
                }
                match list_functions.call_by_type(random, bloc_data, list_global, list_structs, instruction_type, depth) {
                    Some(s) => s,
                    None => return value::random_value(random, instruction_type).to_string(),
                }
            }
            3 => {
                if list_structs.is_empty() {
                    return value::random_value(random, instruction_type).to_string();
                }

                let random_struct = list_structs.get_random(random);
                let struct_type = VarType::strct(random_struct.clone());
                if let Some(var) = bloc_data.get_random_variable(random, vec![&struct_type], matches!(instruction_type, VarType::reference(_))){
                    let name_and_way = var.name_and_way(random, &struct_type);
                    if let Some(s) = random_struct.call_by_type(random, bloc_data, list_global, list_functions, list_structs, instruction_type, depth, Some(name_and_way)) {
                        return s;
                    }
                }

                value::random_value(random, instruction_type).to_string()
            }
            4 => {
                if bloc_data.is_lambdas_empty() {
                    return value::random_value(random, instruction_type).to_string();
                }
                match bloc_data.get_random_lambda(random, vec!(instruction_type.clone())){
                    Some(l) => {
                        let mut is_a_ref = false;
                        let call = l.call(random, bloc_data, list_global, list_functions, list_structs, depth);
                        let way = var_type::way_to_type(random, &l.ret_type().clone().expect("lambda with no ret type"), instruction_type, &mut is_a_ref).expect("no way to type");
                        format!("{}{}{}", if is_a_ref { "*" } else { "" }, call, way)
                    },
                    None => return value::random_value(random, instruction_type).to_string(),
                }
            }
            _ => "".to_string()
        }
    }
}