mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

use std::io::{self, Write};
use std::thread;
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
    let noir_project_name = format!("noir_project{:?}", thread::current().id());
    let noir_project_dir = std::env::current_dir().unwrap().join(noir_project_name);
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
        let code_generated = generate_code::generate_code();
        
        std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

        let compilation_result = nargo_cli::fuzzinglabs_run();

        match compilation_result {
            Ok(_) => {}
            Err(e) => {
                let err = clean_ansi_escape_codes(&e.to_string());
                if !ignored_error(&err) {
                    crash_count += 1;

                    let crash = format!("crash{}", crash_count);

                    std::fs::create_dir_all(&crash_dir.join(&crash)).expect("Failed to create a crash dir");
                    std::fs::copy(&nr_main_path, &crash_dir.join(&crash).join("code.nr")).expect("Failed to copy the main.nr");
                    std::fs::write(&crash_dir.join(&crash).join("err"), &err).expect("Failed to write err");
                }
            }
        }
        loop_count += 1;
        
        print!("\rLoop Count: {} Crash Count: {}", loop_count, crash_count);
        io::stdout().flush().unwrap();
    }

}