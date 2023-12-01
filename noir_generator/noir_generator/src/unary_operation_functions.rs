use rand::Rng;
use crate::variables::Variables;
use crate::statments;

fn get_random_variable() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Only basic variables for now
    let mut string_vec: Vec<String> = Vec::new();
    match index {
        0 => {
            match Variables::get_random_basic_variable() {
                Some(var) => {
                    string_vec.push(var.get_name().to_string());
                    return string_vec
                },
                /* 
                    Error if no variable is found
                    It can only happen if we try to generate a random unary_statment
                    Because the other random variable getters need the type
                */
                None => {
                    if Variables::is_empty() {
                        string_vec.extend(statments::generate_random_variable());
                        string_vec.extend(get_random_variable());
                        return string_vec;
                    } else {
                        return get_random_variable()
                    }
                }
            };
        },
        /*1 => {
            match Variables::get_random_variables_group() {
                Some(v) => return v.get_name().to_string(),
                /* 
                    Error if no variable is found
                    It can only happen if we try to generate a random unary_statment
                    Because the other random variable getters need the type
                */
                None => {
                    if Variables::is_empty() {
                        let result = generate_random_variable();
                        return result + ";\n" + &get_random_variable()
                    } else {
                        return get_random_variable()
                    }
                }
            };
        },*/
        /*2 => {
            match Variables::get_random_reference_variable() {
                Some(v) => return v.get_name().to_string(),
                /* 
                    Error if no variable is found
                    It can only happen if we try to generate a random unary_statment
                    Because the other random variable getters need the type
                */
                None => {
                    if Variables::is_empty() {
                        let result = generate_random_variable();
                        return result + ";\n" + &get_random_variable()
                    } else {
                        return get_random_variable()
                    }
                }
            };
        },*/
        _ => panic!("Unknown index {}", index),
    }
}

fn get_random_variable_except_types(except_types: Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Only basic variables for now
    let mut string_vec: Vec<String> = Vec::new();
    match index {
        0 => {
            match Variables::get_random_variable_except_types(except_types.clone()) {
                Some(var) => {
                    string_vec.push(var.get_name().to_string());
                    return string_vec
                },
                /* 
                    Error if no variable is found
                    It can only happen if we try to generate a random unary_statment
                    Because the other random variable getters need the type
                */
                None => {
                    if !Variables::has_variables_except_types(except_types.clone()) {
                        string_vec.extend(statments::generate_random_variable_except_type(except_types.clone()));
                        string_vec.extend(get_random_variable_except_types(except_types));
                        return string_vec;
                    } else {
                        return get_random_variable_except_types(except_types)
                    }
                }
            };
        },
        /*1 => {},*/
        /*2 => {},*/
        _ => panic!("Unknown index {}", index),
    }
}

fn get_random_variable_with_type(type_: String) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Only basic variables for now
    let mut string_vec: Vec<String> = Vec::new();
    match index {
        0 => {
            match Variables::get_random_variable_with_type(type_.clone()) {
                Some(var) => {
                    string_vec.push(var.get_name().to_string());
                    return string_vec
                },
                /* 
                    Error if no variable is found
                    It can only happen if we try to generate a random unary_statment
                    Because the other random variable getters need the type
                */
                None => {
                    if !Variables::has_variables_with_type(type_.clone()) {
                        string_vec.extend(statments::generate_random_variable_with_type(type_.clone()));
                        string_vec.extend(get_random_variable_with_type(type_));
                        return string_vec;
                    } else {
                        return get_random_variable_with_type(type_)
                    }
                }
            };
        },
        /*1 => {},*/
        /*2 => {},*/
        _ => panic!("Unknown index {}", index),
    }
}

fn verify_type_is_not_str(type_: String) -> bool {
    if type_ == "str" {
        return false;
    }
    return true;
}

pub fn generate_random_unary_operation(operator: String) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    // res_vec.push("// generate_random_unary_statment".to_string());
    match operator.as_str() {
        "" => res_vec.extend(get_random_variable()),
        "!" => res_vec.extend(get_random_variable_with_type("bool".to_string())),
        "-" => res_vec.extend(get_random_variable_except_types(["bool".to_string(), "str".to_string()].to_vec())),
        // "*" => result += &unary_operation_functions::get_random_variable(),
        // "&" => result += &unary_operation_functions::get_random_variable(),
        _ => panic!("Unknown operator {}", operator), // Can't happen
    }
    if let Some(last) = res_vec.last_mut() {
        *last = format!("{}{}", operator, last);
    }
    return res_vec;
}

pub fn generate_random_unary_operation_with_type(type_: String) -> Vec<String>{
    let mut res_vec: Vec<String> = Vec::new();
    // res_vec.push("// generate_random_unary_statment".to_string());
    res_vec.extend(get_random_variable_with_type(type_.clone()));
    if type_ == "bool" {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..2);
        if let Some(last) = res_vec.last_mut() {
            *last = format!("{}{}", if index == 0 {""} else {"!"}, last);
        }
    } else if verify_type_is_not_str(type_){
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..2);
        if let Some(last) = res_vec.last_mut() {
            *last = format!("{}{}", if index == 0 {""} else {"-"}, last);
        }
    }
    return res_vec;
}