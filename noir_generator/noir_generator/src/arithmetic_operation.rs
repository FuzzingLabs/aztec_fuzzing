use rand::Rng;

use crate::{operators, values, Variables, unary_operation_functions, beautifier, binary_operation_functions};

fn generate_arithmetic_operation(operator: String, type_to_operate_on: String) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    // res_vec.push(format!("// generate_arithmetic_operation ({}) with the type : {}", operator, type_to_operate_on));
    match Variables::get_random_variable_with_type(type_to_operate_on.clone()) {
        Some(variable) => {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..2);
            if index == 0 {
                res_vec.extend(unary_operation_functions::generate_random_unary_operation_with_type(type_to_operate_on.clone()));
            } else {
                if type_to_operate_on == "bool" {
                    res_vec.extend(binary_operation_functions::generate_random_binary_operation(true, false, false, vec![type_to_operate_on.clone()]));
                } else {
                    res_vec.extend(binary_operation_functions::generate_random_binary_operation(true, false, true, vec![type_to_operate_on.clone()]));
                }
            }
            match res_vec.pop() {
                Some(last) => {
                    res_vec.push(format!("{} {} {}", variable.get_name(), operator, last));
                }
                None => panic!("Error in generate_arithmetic_operation"),
            }
        },
        None => {
            res_vec.push(format!("// No variable of type {} found", type_to_operate_on));
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..2);
            if index == 0 {
                res_vec.push(Variables::new_random_basic_variable_with_type(type_to_operate_on.clone()));
                res_vec.push(Variables::new_random_basic_variable_with_type(type_to_operate_on.clone()));
                res_vec.extend(generate_arithmetic_operation(operator, type_to_operate_on.clone()))
            } else {
                res_vec.push(Variables::new_random_basic_variable_with_type(type_to_operate_on.clone()));
                res_vec.extend(generate_arithmetic_operation(operator, type_to_operate_on.clone()))
            }
        },
    }
    return beautifier::beautify_vec(res_vec);
}

pub fn generate_random_arithmetic_operation(type_: Vec<String>) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    let type_to_operate_on: String;
    if type_.len() != 0 {
        type_to_operate_on = type_[0].clone();
    } else {
        let except_types: Vec<String> = vec!["str".to_string(), "Field".to_string()];
        type_to_operate_on = values::generate_variable_type_except_types(except_types);
    }
    let arithmetic_operator = operators::generate_arithmetic_operator();
    match arithmetic_operator.as_str() {
        "+" => res_vec.extend(generate_arithmetic_operation("+".to_string(), type_to_operate_on.clone())),
        "-" => res_vec.extend(generate_arithmetic_operation("-".to_string(), type_to_operate_on.clone())),
        "*" => res_vec.extend(generate_arithmetic_operation("*".to_string(), type_to_operate_on.clone())),
        "/" => res_vec.extend(generate_arithmetic_operation("/".to_string(), type_to_operate_on.clone())),
        "%" => res_vec.extend(generate_arithmetic_operation("%".to_string(), type_to_operate_on.clone())),
        _ => panic!("Unknown index {}", arithmetic_operator),
    }
    for i in 0..res_vec.len() {
        res_vec[i] = format!("\t{}", res_vec[i]);
    }
    return res_vec;
}