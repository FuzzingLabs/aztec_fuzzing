mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

use std::io::{self, Read, Write};
use std::path::Path;
use std::{panic, thread};
use rand::Rng;
use gag::{BufferRedirect, Gag};
use noirc_driver;
use noirc_frontend;
use fm::{self, FileId};
use noirc_errors::{self, CustomDiagnostic};

use crate::constants::{MAX_DATA_LENGTH, MIN_DATA_LENGTH};

fn ignored_error(err: &String) -> bool {
    let errors = vec![
        "attempt to divide by zero",
        "Comparisons are invalid on Field types.",
        "Either the operand's type or return type should be specified",
        "expected type",
        "Expected type",
        "The number of bits to use for this bitwise operation is ambiguous.",

        "panicked at",
        "attempt to shift left with overflow",
        "attempt to shift right with overflow"
    ];

    for line in err.lines() {
        if !errors.iter().any(|&e| line.contains(e)) {
            return false;
        }
    }

    true
}

fn clean_ansi_escape_codes(input: &String) -> String {
    let regex = regex::Regex::new(r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]").unwrap();
    regex.replace_all(input, "").into_owned()
}

fn parse_all(fm: &fm::FileManager) -> noirc_frontend::hir::ParsedFiles {
    fm.as_file_map().all_file_ids().map(|&file_id| (file_id, noirc_frontend::hir::def_map::parse_file(fm, file_id))).collect()
}

fn main() {

    let mut loop_count = 0;
    let mut crash_count = 0;

    loop {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(MIN_DATA_LENGTH..=MAX_DATA_LENGTH);
        let vec: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
        let code_generated = generate_code::generate_code(&vec);

        panic::set_hook(Box::new(|panic_info| {
            let err = panic_info.to_string();
            println!("\npanic_info = {}", err);
        }));

        let _ = panic::catch_unwind(|| {
            let mut fm = noirc_driver::file_manager_with_stdlib(Path::new(""));
            fm.add_file_with_source(Path::new(""), code_generated);
            let parsed_files = parse_all(&fm);

            let mut context = noirc_frontend::hir::Context::new(fm, parsed_files);
            let crate_id = noirc_driver::prepare_crate(&mut context, Path::new(""));
            let options = noirc_driver::CompileOptions::default();

            let mut error_msg = String::new();
            match noirc_driver::compile_main(&mut context, crate_id, &options, None){
                Ok(_) => {},
                Err(e) => for error in e.iter() {
                    let str_error = format!("{}", error.diagnostic).lines().next().unwrap().to_string(); // To remove the "secondary:"
                    if error.diagnostic.is_error() && !ignored_error( &str_error) {
                        error_msg = format!("{}\n{}", error_msg, str_error);
                    }
                },
            }

            if !error_msg.is_empty() {
                panic!("{}", error_msg);
            }

        });

        loop_count += 1;
        
        print!("\rLoop Count: {} Crash Count: {}", loop_count, crash_count);
        io::stdout().flush().unwrap();
    }

}