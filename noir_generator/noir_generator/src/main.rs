mod random;
mod generate_function;
mod generate_instruction;
mod bloc_variables;
mod variable;
mod types;

fn main() {
    random::initialize_rng(None);

    let mut code_generated: String = String::new();

    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));
   
   println!("{}", code_generated);
}