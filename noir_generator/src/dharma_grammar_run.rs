mod generate_code;
mod instructions;
mod variables;
mod statements;
mod functions;
mod tools;

use std::process::Command;
use std::io::{self, Write};

use crate::tools::error_management::{clean_ansi_escape_codes, ignored_error_cmd};


/// Generate a code by using dharma with the grammars present in dharma_grammars folder
/// The code generated contain 3 function and a main function
fn generate_code_by_dharma() -> String {
    let mut code_generated = String::new();

    let output = Command::new("dharma")
        .args(&["-grammars", "dharma_grammars/noir_main.dg"])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        code_generated = format!("fn main() {{\n{}}}", String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence"));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Error while using dharma: {}", stderr);
    }

    for i in 1..4 {
        let output = Command::new("dharma")
            .args(&["-grammars", "dharma_grammars/noir_func.dg"])
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            code_generated = format!("fn func_{}() -> i8 {{\n{}}}\n\n{}", i, String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence"), code_generated);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("Error while using dharma: {}", stderr);
        }
    }
    code_generated
}

/// This program will repeatedly call by command line the compiler with randomly generated code using dharma
/// If the compiler raises an error or panic, the program will write the code and the error to the crashes_found folder
fn main() {
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    let crash_dir = std::env::current_dir().unwrap().join("crashes_found");

    if crash_dir.exists() {
        std::fs::remove_dir_all(&crash_dir).expect("Failed to delete old crashes dir");
    }

    std::fs::create_dir_all(&crash_dir).expect("Failed to create the crashes dir");


    let mut loop_count = 0;
    let mut crash_count = 0;

    loop {

        let code_generated = generate_code_by_dharma();

        std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

        let compilation_result = Command::new("nargo")
            .args(&["compile", "--silence-warnings", "--program-dir", noir_project_dir.to_str().unwrap_or_else(|| panic!("Impossible de convertir le chemin en chaîne UTF-8 valide"))])
            .output();

        match compilation_result {
            Ok(output) => {
                if !output.status.success() {
                    let err = clean_ansi_escape_codes(&String::from_utf8_lossy(&output.stderr).to_string());
                    if !ignored_error_cmd(&err) {
                        crash_count += 1;

                        let crash = format!("crash{}", crash_count);

                        std::fs::create_dir_all(&crash_dir.join(&crash)).expect("Failed to create a crash dir");
                        std::fs::copy(&nr_main_path, &crash_dir.join(&crash).join("code.nr")).expect("Failed to copy the main.nr");
                        std::fs::write(&crash_dir.join(&crash).join("err"), &err).expect("Failed to write err");
                    }
                }
            }
            Err(e) => {
                eprintln!("Error executing compilation command: {}", e);
            }
        }
        loop_count += 1;
        
        print!("\rLoop Count: {} Crash Count: {}", loop_count, crash_count);
        io::stdout().flush().unwrap();
    }

}