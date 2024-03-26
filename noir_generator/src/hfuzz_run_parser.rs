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

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < MIN_DATA_LENGTH || data.len() > MAX_DATA_LENGTH {
                return;
            }

            let code_generated = generate_code::generate_code(data);

            let (_, errors) = parser::parse_program(&code_generated);

            if !errors.is_empty() {
                panic!("Parser errors: {:?}", errors);
            }
        })
    }
}
