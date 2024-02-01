mod random;
mod generate_function;
mod instructions;
mod variables;
mod statements;
mod constants;

use std::process::Command;
use std::io::{self, Write};

extern crate honggfuzz;


fn ignored_error(err: &String) -> bool{
    err.contains("attempt to divide by zero")
    || err.contains("Comparisons are invalid on Field types.")
}

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
        random::initialize_rng(None);
        let mut code_generated = String::new();
        for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
            code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
        }
        code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));
        
        std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

        let compilation_result = Command::new("nargo")
            .args(&["compile", "--program-dir", noir_project_dir.to_str().unwrap_or_else(|| panic!("Impossible de convertir le chemin en chaîne UTF-8 valide"))])
            .output();

        match compilation_result {
            Ok(output) => {
                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr).to_string();
                    if !ignored_error(&err) {
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
