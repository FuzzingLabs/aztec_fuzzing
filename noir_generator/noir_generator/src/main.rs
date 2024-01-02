mod random;
mod generate_function;
mod instructions;
mod variables;

use noirc_frontend::parser;

const NB_MAX_FUNCTION: u32 = 10;

fn compile_code() -> Option<Vec<parser::ParserError>> {

    let mut code_generated = String::new();

    for _ in 0..random::gen_range(0, NB_MAX_FUNCTION) {
        code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
    }
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

    random::initialize_rng(Some(12));
    
    let mut errors: Option<Vec<parser::ParserError>> = None;
    let mut compteur = 0;

    while let None = errors {
        errors = compile_code();

        compteur += 1;
        println!("nb loop: {}", compteur);
    }

    for error in &errors {
        println!("{:?}", error);
    }

}
