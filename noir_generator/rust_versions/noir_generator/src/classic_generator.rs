use rand::Rng;

use crate::variable::Variable;
use crate::function::Function;
use crate::value_generator;
use crate::body_generator;

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

fn generate_variable(name: String, visible: bool, mutable: bool, type_: String, value: String) {
    let variable: Variable = Variable::new(name, visible, mutable, type_, value);
    unsafe {
        VARIABLES.push(variable);
    }
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

fn generate_random_variable() -> String {
    let var_name = generate_variable_name();
    let var_visible = false; //generate_bool();
    let var_mutable = generate_bool();
    let var_type = generate_variable_type();
    let var_value = generate_random_value_from_type(var_type.clone());
    let var_group = generate_variable_group();
    let var = format!("{}{}{} {}: {} = {};", var_group, if var_visible { " pub" } else { "" }, if var_mutable { " mut" } else { "" }, var_name, var_type, var_value);
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






pub fn generate() {

    let body: Vec<String> = body_generator::generate_body();

    println!("fn main() {{\n    {}\n}}", body.join("\n    "));

}
