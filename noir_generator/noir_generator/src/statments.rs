use rand::Rng;

use crate::beautifier;
use crate::unary_operation_functions;
use crate::binary_operation_functions;
use crate::variables::Variables;
use crate::operators;

pub fn generate_random_variable() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Only basic variables for now
    match index {
        0 => {
            // res_vec.push("// generate_random_variable".to_string());
            res_vec.push(Variables::new_random_basic_variable());
        },
        //1 => generate_random_variables_group(),
        //2 => generate_random_reference_variable(),
        _ => panic!("Unknown index {}", index),
    }
    return res_vec;
}

pub fn generate_random_variable_except_type(type_: Vec<String>) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Only basic variables for now
    match index {
        0 => {
            // res_vec.push("// generate_random_variable".to_string());
            res_vec.push(Variables::new_random_basic_variable_except_type(type_));
        },
        //1 => generate_random_variables_group(),
        //2 => generate_random_reference_variable(),
        _ => panic!("Unknown index {}", index),
    }
    return res_vec;
}

pub fn generate_random_variable_with_type(type_: String) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    // res_vec.push("// generate_random_variable_with_type".to_string());
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Only basic variables for now
    match index {
        0 => {
            // res_vec.push("// generate_random_variable".to_string());
            res_vec.push(Variables::new_random_basic_variable_with_type(type_));
        },
        //1 => generate_random_variables_group(),
        //2 => generate_random_reference_variable(),
        _ => panic!("Unknown index {}", index),
    }
    return res_vec;
}

fn generate_random_affectation_binary_operation() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    res_vec.push("// generate_random_affectation_binary_operation".to_string());
    return res_vec;
}

fn generate_random_binary_operation_statment() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    res_vec.push("// generate_random_binary_operation_statment".to_string());
    return res_vec;
}

fn generate_random_if_statment() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    res_vec.push("// generate_random_if_statment".to_string());
    return res_vec;
}

fn generate_random_for_statment() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    res_vec.push("// generate_random_for_statment".to_string());
    return res_vec;
}

fn generate_random_call_expression() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    res_vec.push("// generate_random_call_expression".to_string());
    return res_vec;
}

pub fn generate_statment() -> Vec<String> {
    let mut statment_vec: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..3);
    match index {
        0 => statment_vec.extend(unary_operation_functions::generate_random_unary_operation(operators::generate_unary_operator())),
        1 => statment_vec.extend(generate_random_variable()),
        2 => statment_vec.extend(binary_operation_functions::generate_random_binary_operation(false, false, false, Vec::new())),
        //3 => statment_vec.extend(generate_random_affectation_binary_operation(values::generate_affectation_operator())),
        //4 => statment_vec.extend(generate_random_binary_operation_statment(generate_variable_type(), generate_binary_operator())),
        //5 => statment_vec.extend(generate_random_if_statment()),
        /*6 => statment_vec.extend(generate_random_for_statment()),
        7 => statment_vec.extend(generate_random_call_expression()),*/
        _ => panic!("Unknown index {}", index),
    }

    match statment_vec.pop() {
        Some(last) => {
            statment_vec.push(beautifier::beautify_last_line(last));
        }
        None => panic!("Error in generate_arithmetic_operation"),
    }

    return statment_vec;
}

