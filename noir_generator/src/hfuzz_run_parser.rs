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
use crate::constants::{MAX_DATA_LENGTH, MIN_DATA_LENGTH};
use std::io::{self, Read, Write};

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

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < MIN_DATA_LENGTH || data.len() > MAX_DATA_LENGTH {
                return;
            }

            let code_generated = generate_code::generate_code(data);

            let noir_project_dir = std::env::current_dir().unwrap().join("noir_project/src/main.nr");
            std::fs::write(noir_project_dir, &code_generated).expect("Failed to write main.nr");

            let (_, errors) = parser::parse_program(&code_generated);

            if !errors.is_empty() {
                panic!("Parser errors: {:?}", errors);
            }
        })
    }
}
