mod random;
mod generate_function;
mod instructions;
mod variables;
mod statements;
mod constants;

use std::process::Command;

fn fuzz_by_command_line() -> Option<String> {
    let mut code_generated = String::new();
    for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
        code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
    }
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

    let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr";
    std::fs::write(nr_file_path, &code_generated).expect("Failed to write temp file");

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


fn main() {

    random::initialize_rng(None);
    
    let mut errors_cmd: Option<String> = None;
    let mut compteur = 0;

    while errors_cmd.is_none() {

        errors_cmd = fuzz_by_command_line();
        compteur += 1;
        println!("nb loop: {}", compteur);
    }

    match errors_cmd {
        Some(err) => eprintln!("CMD Error: {}", err),
        None => println!("NO CMD Error"),
    }

}
