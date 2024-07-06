#[macro_use]
extern crate honggfuzz;
use noirc_frontend::parser;
use noir_smith::generate_code;

/// This program will run Hongfuzz, only calling the parser
/// with code that is randomly generated using the data provided by Hongfuzz as a source of randomness
fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 10000 || data.len() > 1000000 {
                return;
            }

            let code_generated = generate_code(data);

            let (_, errors) = parser::parse_program(&code_generated);

            if !errors.is_empty() {
                panic!("Parser errors: {:?}", errors);
            }
        })
    }
}
