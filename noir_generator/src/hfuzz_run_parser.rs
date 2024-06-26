#[macro_use]
extern crate honggfuzz;

mod generate_code;
mod instructions;
mod variables;
mod statements;
mod functions;
mod tools;

use noirc_frontend::parser;
use crate::tools::constants::CONFIG;

/// This program will run Hongfuzz, only calling the parser
/// with code that is randomly generated using the data provided by Hongfuzz as a source of randomness
fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < CONFIG.min_data_length || data.len() > CONFIG.max_data_length {
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
