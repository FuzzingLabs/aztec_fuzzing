use rand::Rng;

use crate::{variables::Variables, binary_operation_functions, values, variable::Variable};

pub fn generate_random_affectation_operation() -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    res_vec.push("// generate_random_affectation_operation".to_string());
    let new_and_not_mutate = values::generate_random_value_from_type("bool".to_string());
    let variable_to_affect: Variable;
    if new_and_not_mutate == "true" {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..1); // Only basic variables for now
        match index {
            0 => {
                match Variables::get_name_from_string(Variables::new_random_basic_variable()) {
                    Some(var_name) => {
                        match Variables::get_variable_by_name(var_name.clone()) {
                            Some(var) => {
                                variable_to_affect = var;
                            },
                            None => {
                                panic!("Variable {} not found during random affectation binary operation generation", var_name);
                            }
                        }
                    },
                    None => {
                        // Should never happen
                        panic!("No basic variable found during random affectation binary operation generation")
                    }
                }
            },
            /*1 => variable_to_affect = Variables::new_random_variables_group(),*/
            /*2 => variable_to_affect = Variables::new_random_reference_variable(),*/
            _ => panic!("Unknown index {}", index),
        }
    } else {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..1); // Only basic variables for now
        match index {
            0 => {
                match Variables::get_random_basic_mutable_variable() {
                    Some(var) => {
                        variable_to_affect = var;
                    },
                    None => {
                        // Recall the function until it creates a not mutable variable because we can't find one
                        return generate_random_affectation_operation();
                        // We can also create a new variable and retry until we have a mutable variable
                    }
                }
            },
            /*1 => {},*/
            /*2 => {},*/
            _ => panic!("Unknown index {}", index),
        }
    }
    let binary_operation: Vec<String> = binary_operation_functions::generate_random_binary_operation(true, false, false, vec![variable_to_affect.get_type().to_string()]);
    let mut ret: String = String::new();
    for line in &binary_operation {
        if line != &binary_operation[binary_operation.len() - 1] {
            res_vec.push(line.clone());
        } else {
            ret = line.clone();
        }
    }
    ret = format!("{}{}{} : {} = {}", if new_and_not_mutate == "true" {"let "} else {""}, if variable_to_affect.is_mutable() {"mut "} else {""} , variable_to_affect.get_name(), variable_to_affect.get_type(), ret);
    res_vec.push(ret);

    return res_vec;
}
