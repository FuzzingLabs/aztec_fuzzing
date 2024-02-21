#[macro_use]
extern crate honggfuzz;

mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

use noirc_frontend::parser;

use std::{fs::File, io::Read};
use gag::BufferRedirect;
use nargo_cli;

use crate::constants::{MAX_DATA_LENGTH, MIN_DATA_LENGTH};


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

    let mut crash_count = 0;

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < MIN_DATA_LENGTH || data.len() > MAX_DATA_LENGTH {
                return;
            }

            let code_generated = generate_code::generate_code(data);
            std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

            let mut buf = BufferRedirect::stderr().unwrap();
            let compilation_result = nargo_cli::fuzzinglabs_run(&noir_project_dir);
            let mut err = String::new();
            buf.read_to_string(&mut err).unwrap();
            drop(buf);

            match compilation_result {
                Ok(_) => {
                    return;
                }
                Err(_) => {
                    if ignored_error(&clean_ansi_escape_codes(&err)) {
                        return;
                    }
                    panic!("Error running program: {:?}", err);
                }
            }

        });
    }
}

