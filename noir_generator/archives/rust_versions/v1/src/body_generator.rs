use rand::Rng;

use crate::variable::Variable;
use crate::function::Function;
use crate::value_generator;

static mut VARIABLES: Vec<Variable> = Vec::new();
static mut FUNCTIONS: Vec<Function> = Vec::new();

static mut LAST_VARIABLE_ID: usize = 0;
static mut LAST_FUNCTION_ID: usize = 0;

static POSSIBLE_VARIABLE_TYPES: [&str; 13] = ["Field", "u8", "u16", "u32", "u64", "u127", "i8", "i16", "i32", "i64", "i127", "bool", "str"];
static POSSIBLE_VARIABLE_VISIBILITIES: [&str; 2] = ["pub", ""];
static POSSIBLE_VARIABLE_MUTABILITIES: [&str; 2] = ["mut", ""];
static POSSIBLE_VARIABLE_COMPOUND_TYPES: [&str; 5] = ["Array", "Slice", "Vector", "Tuple", "Struct"];
static POSSIBLE_VARIABLE_GROUPS: [&str; 1] = ["let"];//[&str; 2] = ["let", "const"]; not yet implemented on Noir



fn generate_random_value_from_type(type_: String) -> String {
    match type_.as_str() {
        "Field" => value_generator::generate_random_field(),
        "u8" => value_generator::generate_random_u8(),
        "u16" => value_generator::generate_random_u16(),
        "u32" => value_generator::generate_random_u32(),
        "u64" => value_generator::generate_random_u64(),
        "u127" => value_generator::generate_random_u127(),
        "i8" => value_generator::generate_random_i8(),
        "i16" => value_generator::generate_random_i16(),
        "i32" => value_generator::generate_random_i32(),
        "i64" => value_generator::generate_random_i64(),
        "i127" => value_generator::generate_random_i127(),
        "bool" => value_generator::generate_random_bool(),
        "str" => value_generator::generate_random_str(),
        _ => panic!("Unknown type {}", type_),
    }
}

fn generate_variable_name() -> String {
    unsafe {
        LAST_VARIABLE_ID += 1;
        format!("var_{}", LAST_VARIABLE_ID)
    }
}

fn generate_function_name() -> String {
    unsafe {
        LAST_FUNCTION_ID += 1;
        format!("fn_{}", LAST_FUNCTION_ID)
    }
}

fn generate_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn generate_variable_type() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..POSSIBLE_VARIABLE_TYPES.len());
    POSSIBLE_VARIABLE_TYPES[index].to_string()
}

fn generate_variable_group() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..POSSIBLE_VARIABLE_GROUPS.len());
    POSSIBLE_VARIABLE_GROUPS[index].to_string()
}

fn generate_variable(name: String, visible: bool, mutable: bool, type_: String, value: String) -> String{
    let variable: Variable = Variable::new(name, visible, mutable, type_, value);
    unsafe {
        VARIABLES.push(variable.clone());
    }
    variable.to_string()
}

fn get_variables() -> &'static Vec<Variable> {
    unsafe {
        &VARIABLES
    }
}

fn get_variable(name: String) -> &'static Variable {
    unsafe {
        for variable in &VARIABLES {
            if variable.get_name() == &name {
                return variable;
            }
        }
        panic!("Variable {} not found", name);
    }
}

fn generate_function(name: String, visible: bool, return_type: String, parameters: Vec<String>, body: Vec<String>) {
    let function: Function = Function::new(name, visible, return_type, parameters, body);
    unsafe {
        FUNCTIONS.push(function);
    }
}

fn get_functions() -> &'static Vec<Function> {
    unsafe {
        &FUNCTIONS
    }
}

fn get_function(name: String) -> &'static Function {
    unsafe {
        for function in &FUNCTIONS {
            if function.get_name() == &name {
                return function;
            }
        }
        panic!("Function {} not found", name);
    }
}

fn generate_random_variable() -> Variable {
    let var_name = generate_variable_name();
    let var_visible = false; //generate_bool();
    let var_mutable = generate_bool();
    let var_type = generate_variable_type();
    let var_value = generate_random_value_from_type(var_type.clone());
    let var_group = generate_variable_group(); // this has to be let because const is not yet implemented on Noir
    let var: Variable = Variable::new(var_name, var_visible, var_mutable, var_type, var_value);
    unsafe {
        VARIABLES.push(var.clone());
    }
    var
}

fn generate_random_function() -> String {
    let fn_name = generate_function_name();
    let fn_visible = generate_bool();
    let fn_return_type = generate_variable_type();
    let fn_parameters = vec!["a".to_string(), "b".to_string()];
    let fn_body = vec!["a + b".to_string()];
    let fn_ = format!("{}fn {}({}) -> {} {{\n    {}\n}}", if fn_visible { "pub " } else { "" }, fn_name, fn_parameters.join(", "), fn_return_type, fn_body.join("\n    "));
    fn_
}


fn generate_affectation_binary_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..1); // Not (0..11) because not yet implemented on Noir
    match index {
        0 => "=".to_string(),
        1 => "+=".to_string(),
        2 => "-=".to_string(),
        3 => "*=".to_string(),
        4 => "/=".to_string(),
        5 => "%=".to_string(),
        6 => "&=".to_string(),
        7 => "|=".to_string(),
        8 => "^=".to_string(),
        9 => "<<=".to_string(),
        10 => ">>=".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

fn generate_binary_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..10);
    match index {
        0 => "+".to_string(),
        1 => "-".to_string(),
        2 => "*".to_string(),
        3 => "/".to_string(),
        4 => "%".to_string(),
        5 => "&".to_string(),
        6 => "|".to_string(),
        7 => "^".to_string(),
        8 => "<<".to_string(),
        9 => ">>".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

fn generate_comparison_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..6);
    match index {
        0 => "==".to_string(),
        1 => "!=".to_string(),
        2 => ">=".to_string(),
        3 => "<=".to_string(),
        4 => ">".to_string(),
        5 => "<".to_string(),
        _ => panic!("Unknown index {}", index),
    }
}

fn generate_random_unary_operator() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..3);
    match index {
        0 => "".to_string(),
        1 => "!".to_string(),
        2 => "-".to_string(),
        // 3 => "*".to_string(), // TODO: In development
        // 4 => "&".to_string(), // This one is not working
        _ => panic!("Unknown index {}", index),
    }
}

fn get_random_variable() -> Variable {
    let variables: &Vec<Variable> = get_variables();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..variables.len());
    variables[index].clone()
}

fn get_random_variable_except_types(except_types: [String; 2]) -> Variable {
    let variables: &Vec<Variable> = get_variables();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..variables.len());
    let mut var = variables[index].clone();
    while except_types.contains(&var.get_type().to_string()) {
        var = get_random_variable_except_types(except_types.clone())
    }
    var
}

fn verify_variable_type(type_: String) -> bool {
    let variables: &Vec<Variable> = get_variables();
    for variable in variables {
        if variable.get_type() == type_ {
            return true;
        }
    }
    false
}

fn get_random_variable_with_type(type_: String) -> Variable {
    let variables: &Vec<Variable> = get_variables();
    // verifier qu'au moins une variable a le type
    if verify_variable_type(type_.clone()) {
        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(0..variables.len());
        while variables[index].get_type() != type_ {
            index = rng.gen_range(0..variables.len());
        }
        variables[index].clone()
    } else {
        panic!("No variable with type {}", type_);
    }
}

fn get_random_mutable_variable() -> Variable {
    let variables: &Vec<Variable> = get_variables();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..variables.len());
    let mut var = variables[index].clone();
    while !var.is_mutable() {
        var = get_random_mutable_variable();
    }
    var
}

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

fn generate_statment() -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..5);
    match index {
        0 => generate_unary_statment() + ";\n",
        1 => generate_random_variable().to_string() + ";\n",
        2 => generate_random_affectation_binary_operation(generate_affectation_binary_operator()) + ";\n",
        3 => generate_random_binary_operation_statment(generate_variable_type(), generate_binary_operator()) + ";\n",
        4 => generate_random_if_statment() + "\n",
        /*5 => generate_random_for_statment() + "\n",
        6 => generate_random_call_expression() + "\n",*/
        _ => panic!("Unknown index {}", index),
    }
}



pub fn generate() {
    println!("classic");

    println!("Variables:");
    for i in 0..10 {
        println!("{}", generate_random_variable().to_string());
    }
    
    println!("Functions:");
    for i in 0..10 {
        println!("{}", generate_random_function());
    }
}

pub fn generate_body() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    
    for i in 0..100 {
        result.push(generate_random_variable().to_string()+";");

    }

    for i in 0..100 {
        result.push(generate_statment());
    }
    result
}