mod random;
mod generate_function;
mod instructions;
mod variables;
mod statements;

use noirc_frontend::hir::def_collector::dc_crate::CompilationError;
use noirc_frontend::hir::resolution::errors::ResolverError;
use fm::FileId;

use noirc_frontend::tests::test::get_program_errors;


const NB_MAX_FUNCTION: u32 = 10;

fn fuzz() -> Option<Vec<(CompilationError, FileId)>> {
    let mut code_generated = String::new();
    for _ in 0..random::gen_range(0, NB_MAX_FUNCTION) {
        code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
    }
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));
    
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
    
    let mut errors_opt: Option<Vec<(CompilationError, FileId)>> = None;
    let mut compteur = 0;

    while let None = errors_opt {
        
        errors_opt = fuzz();

        compteur += 1;
        println!("nb loop: {}", compteur);
    }

    errors_opt.map(|errors| {
        for (error, _) in errors {
            println!("Error: {:?}", error);
        }
    });

}
