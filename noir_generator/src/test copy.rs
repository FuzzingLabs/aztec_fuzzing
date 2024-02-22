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
        "The number of bits to use for this bitwise operation is ambiguous."
    ];

    for line in err.lines() {
        if !errors.iter().any(|&e| line.contains(e)) {
            println!("this line is false : {}", line);
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
    let noir_project_name = format!("noir_project");
    let noir_project_dir = std::env::current_dir().unwrap().join(noir_project_name);
    let nr_main_path = noir_project_dir.join("src/main.nr");

    let mut loop_count = 0;
    let mut crash_count = 0;

    loop {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(MIN_DATA_LENGTH..=MAX_DATA_LENGTH);
        let vec: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
        let code_generated = generate_code::generate_code(&vec);

        panic::set_hook(Box::new(|panic_info| {
            let err = panic_info.to_string();
            if !ignored_error(&err){
                println!("panic_info = {}", err);
            }
        }));

        let _ = panic::catch_unwind(|| {
        let mut error_msg = String::new();
        let mut fm = noirc_driver::file_manager_with_stdlib(noir_project_dir.as_path());
        fm.add_file_with_source(Path::new("src/main.nr"), code_generated);
        let parsed_files = parse_all(&fm);

        let mut context = noirc_frontend::hir::Context::new(fm, parsed_files);
        let crate_id = noirc_driver::prepare_crate(&mut context, nr_main_path.as_path());
        let options = noirc_driver::CompileOptions::default();

        match noirc_driver::check_crate(&mut context, crate_id, false, options.disable_macros){
            Ok(_) => {},
            Err(e) => for error in e.iter() {
                let str_error = format!("{}", error.diagnostic).lines().next().unwrap().to_string();
                if error.diagnostic.is_error() && !ignored_error( &str_error) {
                    error_msg = format!("{}{}", error_msg, str_error);
                }
            },
        }

        let main = context.get_main_function(&crate_id).ok_or_else(|| { 
            // TODO(#2155): This error might be a better to exist in Nargo
            let err = CustomDiagnostic::from_message(
                "cannot compile crate into a program as it does not contain a `main` function",
            )
            .in_file(FileId::default());
            vec![err]
        });
        
        match noirc_driver::compile_no_check(&mut context, &options, main.expect("fdgdf"), None, options.force_compile){
            Ok(_) => {},
            Err(e) => error_msg = format!("{}{}", error_msg, e),
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