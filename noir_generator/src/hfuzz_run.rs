#[macro_use]
extern crate honggfuzz;
extern crate toml;

mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;
mod tools;


use fm::FileManager;
use nargo::ops::compile_workspace;
use nargo_toml::{resolve_workspace_from_toml, PackageSelection};
use noirc_driver::{file_manager_with_stdlib, CompileOptions, NOIR_ARTIFACT_VERSION_STRING};
use noirc_frontend::{hir::{def_map::parse_file, ParsedFiles}, parse_program};

use crate::constants::CONFIG;
use crate::tools::ignored_error;

fn parse_all(fm: &FileManager) -> ParsedFiles {
    fm.as_file_map().all_file_ids().map(|&file_id| (file_id, parse_file(fm, file_id))).collect()
}

fn main() {
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    let fm_stdlib = &file_manager_with_stdlib(std::path::Path::new(""));
    let parsed_files_stdlib = parse_all(&fm_stdlib);

    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < CONFIG.min_data_length || data.len() > CONFIG.max_data_length {
                return;
            }

            let code_generated = generate_code::generate_code(data);

            let mut fm = fm_stdlib.clone();
            let mut parsed_files = parsed_files_stdlib.clone();
            
            let parsed = parse_program(&code_generated);
            let file_id = fm.add_file_with_source(&nr_main_path, code_generated.clone());
            parsed_files.insert(file_id.expect("No file id"), parsed);


            let options = CompileOptions::default();

            let workspace = match resolve_workspace_from_toml(
                &noir_project_dir.join("Nargo.toml"),
                PackageSelection::DefaultOrAll,
                Some(NOIR_ARTIFACT_VERSION_STRING.to_string()),
            ) {
                Ok(w) => w,
                Err(_) => panic!("Can't resolve workspace from toml"),
            };

            match compile_workspace(&fm, &parsed_files, &workspace, &options) {
                Ok(_) => return,
                Err(errors) => for error in errors.iter() {
                    if error.diagnostic.is_error() && !ignored_error(&error.diagnostic.message) {
                        panic!("{}", error.diagnostic.message);
                    }
                },
            }
        });
    }
}

