use rand::Rng;

use crate::arithmetic_operation;
use crate::beautifier;
use crate::beautifier::beautify_vec;
use crate::operators::generate_affectation_operator;
use crate::values;
use crate::operators;
use crate::variable::Variable;
use crate::variables::Variables;

use crate::affectation_operation;

pub fn generate_random_comparison_operation(type_: Vec<String>) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    if type_.len() != 0 {
        res_vec.push(format!("// generate_random_comparison_operation of type : {}", type_[0]));
    } else {
        res_vec.push("// generate_random_comparison_operation".to_string());
    }
    return res_vec;
}

pub fn generate_random_logical_operation(type_: Vec<String>) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    if type_.len() != 0 {
        res_vec.push(format!("// generate_random_logical_operation of type : {}", type_[0]));
    } else {
        res_vec.push("// generate_random_logical_operation".to_string());
    }
    return res_vec;
}

pub fn generate_random_bitwise_operation(type_: Vec<String>) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    if type_.len() != 0 {
        res_vec.push(format!("// generate_random_bitwise_operation of type : {}", type_[0]));
    } else {
        res_vec.push("// generate_random_bitwise_operation".to_string());
    }
    return res_vec;
}

pub fn generate_random_binary_operation(except_affectation_operation: bool, only_affectation_operation: bool, only_numerical_operation: bool, type_: Vec<String>) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    // res_vec.push("// generate_random_binary_operation".to_string());
    let mut rng = rand::thread_rng();
    let mut index: usize;
    if only_affectation_operation {
        index = 4;
    } else if only_numerical_operation {
        // Either arithmetic or bitwise operation
        index = rng.gen_range(0..2);
        if index == 1 {
            index = 3;
        }
    } else if type_.len() == 0 {
        if except_affectation_operation {
            index = rng.gen_range(0..4);
        } else {
            index = rng.gen_range(0..5);
        }
    } else {
        if Variables::type_is_str(type_[0].clone()) {
            if except_affectation_operation {
                index = 1;
            } else {
                // Either comparison or affectation operation
                index = rng.gen_range(0..2);
                if index == 0 {
                    index = 1;
                } else {
                    index = 4;
                }

            }
        } else if type_[0] == "Field" {
            if except_affectation_operation {
                // Either arithmetic or comparison operation
                index = rng.gen_range(0..2);
            } else {
                // Either arithmetic or comparison or affectation operation
                index = rng.gen_range(0..3);
                if index == 2 {
                    index = 4;
                }
            }
        } else if except_affectation_operation {
            index = rng.gen_range(0..4);
        } else {
            index = rng.gen_range(0..5);
        }
    }
    match index {
        0 => res_vec.extend(arithmetic_operation::generate_random_arithmetic_operation(type_.clone())),
        1 => res_vec.extend(generate_random_comparison_operation(type_.clone())),
        2 => res_vec.extend(generate_random_logical_operation(type_.clone())),
        3 => res_vec.extend(generate_random_bitwise_operation(type_.clone())),
        4 => res_vec.extend(affectation_operation::generate_random_affectation_operation()),
        _ => panic!("Unknown index {}", index),
    }
    if type_.len() != 0 {
        match res_vec.pop() {
            Some(last) => {
                res_vec.push(beautifier::beautify_last_line(format!("{} as {}", beautifier::beautify_as_type(last, type_[0].to_string()), type_[0])));
            }
            None => panic!("Error in generate_arithmetic_operation"),
        }
    }
    return res_vec;
}
