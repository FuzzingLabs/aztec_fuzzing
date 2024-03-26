#[macro_use]
extern crate honggfuzz;

mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;
mod tools;

use std::process::Command;

use crate::{constants::{MAX_DATA_LENGTH, MIN_DATA_LENGTH}, tools::{clean_ansi_escape_codes, ignored_error}};

fn main() {

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < MIN_DATA_LENGTH || data.len() > MAX_DATA_LENGTH {
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
                        if ignored_error(&err) {
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
