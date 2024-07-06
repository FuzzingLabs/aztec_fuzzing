#[macro_use]
extern crate honggfuzz;
use std::process::Command;
use noir_smith::generate_code;
mod error_management;

/// This program will run Hongfuzz, calling by command line the compiler
/// with code that is randomly generated using the data provided by Hongfuzz as a source of randomness
fn main() {

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 10000 || data.len() > 1000000 {
                return;
            }
            
            let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
            let nr_main_path = noir_project_dir.join("src/main.nr");

            let code_generated = generate_code(data);
            std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

            match Command::new("nargo")
            .args(&["compile", "--silence-warnings", "--program-dir", noir_project_dir.to_str().unwrap_or_else(|| panic!("Impossible de convertir le chemin en chaîne UTF-8 valide"))])
            .output() {
                Ok(output) => {
                    if !output.status.success() {
                        let err = error_management::clean_ansi_escape_codes(&String::from_utf8_lossy(&output.stderr).to_string());
                        if error_management::ignored_error_cmd(&err) {
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
