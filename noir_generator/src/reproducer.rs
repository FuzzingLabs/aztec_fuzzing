mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

use std::process::Command;
use nargo_cli;

fn ignored_error(err: &String) -> bool {
    let errors = vec![
        "attempt to divide by zero",
        "Comparisons are invalid on Field types.",
        "Either the operand's type or return type should be specified",
        "expected type",
        "Expected type",
        "The number of bits to use for this bitwise operation is ambiguous."
    ];

    for line in err.lines() {
        if line.contains("error:") {
            if !errors.iter().any(|&e| line.contains(e)) {
                return false;
            }
        }
    }

    true
}

fn clean_ansi_escape_codes(input: &String) -> String {
    let regex = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]").unwrap();
    regex.replace_all(input, "").into_owned()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path as a command line argument.");
        std::process::exit(1);
    }
    let file_path = &args[1];
    let data = std::fs::read(file_path).expect("Could not read file");

    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    random::initialize_rng(Some(&data));
    let code_generated = generate_code::generate_code();

    std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");
    
    let compilation_result = nargo_cli::fuzzinglabs_run(&noir_project_dir);

    match compilation_result {
        Ok(_) => {
        }
        Err(e) => {
            let err = clean_ansi_escape_codes(&e.to_string());
            if !ignored_error(&err) {
                println!("REAL CRASH WITH COMPILATION BY FUNCTION CALL");
                return;
            }
        }
    }

    let compilation_result = Command::new("nargo")
            .args(&["compile", "--program-dir", noir_project_dir.to_str().unwrap_or_else(|| panic!("Impossible de convertir le chemin en chaîne UTF-8 valide"))])
            .output();

    match compilation_result {
        Ok(output) => {
            if !output.status.success() {
                let err = clean_ansi_escape_codes(&String::from_utf8_lossy(&output.stderr).to_string());
                if !ignored_error(&err) {
                    println!("{}", err);
                    println!("REAL CRASH WITH COMPILATION BY COMMAND");
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Error executing compilation command: {}", e);
        }
    }

    println!("NOT A REAL CRASH");

}
