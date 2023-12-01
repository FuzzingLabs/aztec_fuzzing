use crate::variable::Variable;
use crate::function::Function;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref VARIABLES: Mutex<Vec<Variable>> = Mutex::new(Vec::new());
}

fn get_variables() -> Result<MutexGuard<'static, Vec<Variable>>, &'static str> {
    VARIABLES.lock().map_err(|_| "Failed to acquire lock on VARIABLES")
}

pub fn generate_variable(name: String, visibility: bool, type_: String, value: String) {
    let variable: Variable = Variable::new(name, visibility, type_, value);
    if let Ok(mut variables) = get_variables() {
        variables.push(variable);
        // variables is automatically released here
    } else {
        eprintln!("Failed to obtain lock on VARIABLES");
    }
}

pub fn generate() {
    println!("classic");

    match get_variables() {
        Ok(variables) => {
            println!("\nVariables:");
            for variable in &*variables {
                println!("\n{}", variable);
            }
        }
        Err(e) => {
            eprintln!("Error obtaining lock on VARIABLES: {}", e);
            return;
        }
    }

    generate_variable("a".to_string(), true, "i32".to_string(), "5".to_string());

    match get_variables() {
        Ok(variables) => {
            println!("\nAfter adding 'a':");
            for variable in &*variables {
                println!("\n{}", variable);
            }
        }
        Err(e) => {
            eprintln!("Error obtaining lock on VARIABLES: {}", e);
            return;
        }
    }

    generate_variable("b".to_string(), false, "i64".to_string(), "10".to_string());

    match get_variables() {
        Ok(variables) => {
            println!("\nAfter adding 'b':");
            for variable in &*variables {
                println!("\n{}", variable);
            }
        }
        Err(e) => {
            eprintln!("Error obtaining lock on VARIABLES: {}", e);
            return;
        }
    }

    let func1: Function = Function::new(
        "func1".to_string(),
        true,
        "i32".to_string(),
        vec!["a: i32".to_string(), "b: i32".to_string()],
        vec!["a + b".to_string()],
    );
    let func2: Function = Function::new(
        "func2".to_string(),
        false,
        "i64".to_string(),
        vec!["a: i64".to_string(), "b: i64".to_string()],
        vec!["a + b".to_string()],
    );
    println!("\n{}", func1);
    println!("\n{}", func2);
}
