mod random;
mod generate_function;
mod instructions;
mod variables;
mod statements;
mod constants;

use std::process::Command;

use noirc_frontend::hir::def_collector::dc_crate::CompilationError;
use noirc_frontend::hir::resolution::errors::ResolverError;
use fm::FileId;

use noirc_frontend::tests::test::get_program_errors;

fn fuzz_by_command_line() -> Option<String> {
    let mut code_generated = String::new();
    for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
        code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
    }
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

    let compilation_result = Command::new("/home/afredefon/.nargo/bin/nargo")
        .args(&["compile", "--program-dir", "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test", "--package", "test"])
        .output();

    match compilation_result {
        Ok(output) => {
            if output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => {
            eprintln!("Error executing compilation command: {}", e);
            None
        }
    }
}

fn fuzz_by_source_code(code_generated: String) -> Option<Vec<(CompilationError, FileId)>> {
    
    let mut errors = get_program_errors(&code_generated);

    errors.retain(|(err,_)| match err {
        CompilationError::ResolverError(ResolverError::UnusedVariable{..}) => false,
        _ => true,
    });

    if errors.is_empty() {
        return None
    }

    let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr";
    std::fs::write(nr_file_path, &code_generated).expect("Failed to write temp file");
    Some(errors)
}


fn main() {

    random::initialize_rng(None);
    
    let mut errors_src: Option<Vec<(CompilationError, FileId)>> = None;
    let mut errors_cmd: Option<String> = None;
    let mut compteur = 0;

    while errors_src.is_none() && errors_cmd.is_none() {
        let mut code_generated = String::new();
        for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
            code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
        }
        code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

        let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr";
        std::fs::write(nr_file_path, &code_generated).expect("Failed to write temp file");
        
        errors_src = fuzz_by_source_code(code_generated);
        errors_cmd = fuzz_by_command_line();

        compteur += 1;
        println!("nb loop: {}", compteur);
    }

    match errors_cmd {
        Some(err) => eprintln!("CMD Error: {}", err),
        None => println!("NO CMD Error"),
    }

    errors_src.map(|errors| {
        for (error, _) in errors {
            println!("SRC Error: {:?}", error);
        }
    });

}
