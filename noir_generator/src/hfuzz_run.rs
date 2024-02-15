#[macro_use]
extern crate honggfuzz;

mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

use nargo_cli;


fn ignored_error(err: &str) -> bool {
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
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 8 {
                return;
            }
            random::initialize_rng(Some(data));
            let code_generated = generate_code::generate_code();
            std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

            match nargo_cli::fuzzinglabs_run(&noir_project_dir) {
                Ok(_) => {
                    return;
                }
                Err(e) => {
                    let err = clean_ansi_escape_codes(&e.to_string());
                    if ignored_error(&err) {
                        return;
                    }
                    panic!("Error running program: {:?}", err);
                    
                }
            }

        });
    }
}
