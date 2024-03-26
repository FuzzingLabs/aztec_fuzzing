#[macro_use]
extern crate honggfuzz;

mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;
mod tools;

use std::path::Path;


use crate::{constants::{MAX_DATA_LENGTH, MIN_DATA_LENGTH}, tools::ignored_error};

fn parse_all(fm: &fm::FileManager) -> noirc_frontend::hir::ParsedFiles {
    fm.as_file_map().all_file_ids().map(|&file_id| (file_id, noirc_frontend::hir::def_map::parse_file(fm, file_id))).collect()
}

fn main() {

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < MIN_DATA_LENGTH || data.len() > MAX_DATA_LENGTH {
                return;
            }

            let code_generated = generate_code::generate_code(data);

            let mut fm = noirc_driver::file_manager_with_stdlib(Path::new(""));
            fm.add_file_with_source(Path::new(""), code_generated);
            let parsed_files = parse_all(&fm);

            let mut context = noirc_frontend::hir::Context::new(fm, parsed_files);
            let crate_id = noirc_driver::prepare_crate(&mut context, Path::new(""));
            let options = noirc_driver::CompileOptions::default();

            let mut error_msg = String::new();
            match noirc_driver::compile_main(&mut context, crate_id, &options, None){
                Ok(_) => return,
                Err(e) => for error in e.iter() {
                    let str_error = format!("{}", error.diagnostic).lines().next().unwrap().to_string(); // To remove the "secondary:"
                    if error.diagnostic.is_error() && !ignored_error( &str_error) {
                        error_msg = format!("{}\n{}", error_msg, str_error);
                    }
                },
            }

            if error_msg.is_empty() {
                return
            }

            panic!("{}", error_msg);
        });
    }
}

