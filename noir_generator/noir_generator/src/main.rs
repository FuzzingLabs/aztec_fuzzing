mod random;
mod generate_function;
mod generate_instruction;
mod bloc_variables;
mod variable;
mod types;

use std::process::Command;
use honggfuzz::fuzz;

// Function to process the code
fn process_code(data: &[u8]) {

    if let Some(seed) = data.get(0).cloned() {
        random::initialize_rng(Some(seed));
    } else {
        random::initialize_rng(None);
    }

    let mut code_generated = String::new();
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

    let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr"; // Choisissez un chemin approprié
    std::fs::write(nr_file_path, code_generated).expect("Failed to write temp file");

    // Execute the compilation command
    let compilation_result = Command::new("/home/afredefon/.nargo/bin/nargo")
        .args(&["compile", "--program-dir", "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test", "--package", "test"])
        .output();

    // Check the result of the compilation
    match compilation_result {
        Ok(output) => {
            if output.status.success() {
                println!("Compilation succeeded");
            } else {
                eprintln!("Compilation failed:\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Error executing compilation command: {}", e);
        }
    }

    // Clean up resources
    std::fs::remove_file(nr_file_path).expect("Failed to remove temp file");
}

fn main() {

    // Fuzz the code generation and compilation process
    fuzz(|data: &[u8]| {

        // Invoke the synchronized process_code function
        process_code(data);
    });
}
