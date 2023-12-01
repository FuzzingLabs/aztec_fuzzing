use crate::variables::Variables;

#[macro_use]
extern crate lazy_static;

mod variable;
mod variables;
mod values;
mod operators;
mod statments;
mod unary_operation_functions;
mod binary_operation_functions;
mod arithmetic_operation;
mod affectation_operation;
mod beautifier;

/*

fn generate_random_binary_operation_statment(type_: String, binary_operator: String) -> String {
    let variable1: Variable = get_random_variable_with_type(type_.clone());
    let variable2: Variable = get_random_variable_with_type(type_.clone());
    let mut binary_operation: String = String::new();
    if ["%", ">", ">>", "<<", "^"].contains(&binary_operator.as_str()) {
        binary_operation = format!("{} {} {} as {}", variable1.get_name(), binary_operator, variable2.get_name(), type_);
    } else {
        binary_operation = format!("{} {} {}", variable1.get_name(), binary_operator, variable2.get_name());
    }
    binary_operation
}

fn generate_random_affectation_binary_operation(affectation_binary_operator: String) -> String {
    let use_mutable_variable = generate_bool();
    let binary_operator = generate_binary_operator();
    if use_mutable_variable {
        let mut variable: Variable = get_random_mutable_variable();
        let binary_operation = generate_random_binary_operation_statment(variable.get_type().to_string(), binary_operator);
        let affectation_binary_operation = format!("{} {} {}", variable.get_name(), affectation_binary_operator, binary_operation);
        return affectation_binary_operation;
    } else {
        let mut variable: Variable = generate_random_variable();
        let binary_operation = generate_random_binary_operation_statment(variable.get_type().to_string(), binary_operator);
        let affectation_binary_operation = format!("let {} {} {}", variable.get_name(), affectation_binary_operator, binary_operation);
        return affectation_binary_operation;
    }
}

fn generate_random_condition() -> String {
    let comparison_operator = generate_comparison_operator();
    let variable1: Variable = get_random_variable();
    let variable2: Variable = get_random_variable_with_type(variable1.get_type().to_string());
    let condition: String = format!("{} {} {}", variable1.get_name(), comparison_operator, variable2.get_name());
    condition
}

fn generate_random_if_statment_without_else() -> String {
    let condition = generate_random_condition();
    let mut rng = rand::thread_rng();
    let body_size = rng.gen_range(0..10);
    let mut body: Vec<String> = Vec::new();
    for i in 0..body_size {
        body.push("\t".to_owned() + &generate_unary_statment() + ";");
    }
    let if_statment: String = format!("if {} {{\n    {}\n    }}", condition, body.join("\n    "));
    if_statment 
}

fn generate_random_if_statment_with_else() -> String {
    let if_without_else = generate_random_if_statment_without_else();
    let mut rng = rand::thread_rng();
    let else_size = rng.gen_range(0..10);
    let mut else_body: Vec<String> = Vec::new();
    for i in 0..else_size {
        else_body.push("\t".to_owned() + &generate_unary_statment() + ";");
    }
    let if_statment: String = format!("{} else {{\n    {}\n    }}", if_without_else, else_body.join("\n    "));
    if_statment
}

fn generate_random_if_statment() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..2);
    match index {
        0 => generate_random_if_statment_with_else(),
        1 => generate_random_if_statment_without_else(),
        _ => panic!("Unknown index {}", index),
    }
}

fn get_random_reference_variable() -> Variable {
    let variables: &Vec<Variable> = get_variables();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..variables.len());
    let mut var = variables[index].clone();
    while var.get_type() != "str" {
        var = get_random_reference_variable();
    }
    var
}

fn generate_unary_statment() -> String {
    let unary_operator = generate_random_unary_operator();
    if unary_operator == "" {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..2);
        match index {
            0 => {
                let variable: Variable = get_random_variable();
                let unary_statment: String = format!("{}", variable.get_name());
                unary_statment
            },
            1 => {
                let binary_operation: String = generate_random_binary_operation_statment(generate_variable_type(), generate_binary_operator());
                binary_operation
            },
            _ => panic!("Unknown index {}", index),
        }
    } else {        
        if unary_operator == "*" {
            let mut variable: Variable = get_random_reference_variable();
            return format!("{}{}; // unary_operator: {} booltest passed ok", unary_operator, variable.get_name(), unary_operator);
        } else if unary_operator == "!".to_string() {
            let mut variable: Variable = get_random_variable_with_type("bool".to_string());
            return format!("{}{}", unary_operator, variable.get_name());
        } else {
            let mut variable: Variable = get_random_variable_except_types(["bool".to_string(), "str".to_string()]);
            return format!("{}{}", unary_operator, variable.get_name());
        }
    }
    
}

*/

fn main() {
    /*
    println!("fn main() {{");
    for _ in 0..1 {
        println!("\t{}", Variables::new_random_basic_variable() + ";");
    }
    for _ in 0..5 {
        println!("{}", beautifier::beautify_statment(binary_operation_functions::generate_random_binary_operation(false, false, Vec::new()), true));
    }
    println!("}}");
    */

    println!("fn main() {{");

    for _ in 0..20 {
        println!("\t{}", Variables::new_random_basic_variable() + ";");
    }

    for _ in 0..5000 {
        println!("_____________________________________");
        println!("{}", statments::generate_statment(false));
    }

    println!("}}")
}
