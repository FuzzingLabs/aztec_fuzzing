mod random;
mod generate_function;
mod generate_instruction;
mod bloc_variables;
mod variable;
mod types;

use noirc_frontend::parser;

fn compile_code() -> Option<Vec<parser::ParserError>> {

    let mut code_generated = String::new();
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

    let (_, errors) = parser::parse_program(&code_generated);

    if errors.len() != 0 {
        let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr"; // Choisissez un chemin approprié
        std::fs::write(nr_file_path, code_generated).expect("Failed to write temp file");
        return Some(errors);
    }

    None
}

fn main() {

    random::initialize_rng(None);
    
    let mut errors: Option<Vec<parser::ParserError>> = None;

    while let None = errors {
        errors = compile_code();
    }

    for error in &errors {
        println!("{:?}", error);
    }

}
