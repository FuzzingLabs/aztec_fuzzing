#[macro_use]
extern crate honggfuzz;

mod generate_code;
mod instructions;
mod variables;
mod statements;
mod functions;
mod tools;

use std::process::Command;

use crate::{tools::constants::CONFIG, tools::error_management::{clean_ansi_escape_codes, ignored_error_cmd}};

/// This program will run Hongfuzz, calling by command line the compiler
/// with code that is randomly generated using the data provided by Hongfuzz as a source of randomness
fn main() {

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < CONFIG.min_data_length || data.len() > CONFIG.max_data_length {
                return;
            }
            
            let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
            let nr_main_path = noir_project_dir.join("src/main.nr");

            let code_generated = generate_code::generate_code(data);
            std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

            match Command::new("nargo")
            .args(&["compile", "--silence-warnings", "--program-dir", noir_project_dir.to_str().unwrap_or_else(|| panic!("Impossible de convertir le chemin en chaîne UTF-8 valide"))])
            .output() {
                Ok(output) => {
                    if !output.status.success() {
                        let err = clean_ansi_escape_codes(&String::from_utf8_lossy(&output.stderr).to_string());
                        if ignored_error_cmd(&err) {
                            return;
                        }
                        panic!("Error : {:?}", err);
                    } else {
                        return;
                    }
                }
                Err(e) => {
                    panic!("Error executing compilation command: {}", e);
                }
            }

        });
    }
}
