mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;
mod tools;

use std::io::{self, Write};
use fm::FileManager;
use nargo::ops::compile_workspace;
use noirc_driver::{file_manager_with_stdlib, NOIR_ARTIFACT_VERSION_STRING};
use noirc_frontend::hir::def_map::parse_file;
use noirc_frontend::hir::ParsedFiles;
use rand::Rng;

use std::path::Path;
use noirc_frontend::parser;
use nargo_toml::{resolve_workspace_from_toml, PackageSelection};

use crate::constants::CONFIG;
use crate::tools::ignored_error;


fn parse_all(fm: &FileManager) -> ParsedFiles {
    let ret = fm.as_file_map().all_file_ids().map(|&file_id| (file_id, parse_file(fm, file_id))).collect();
    ret
}

fn main() {
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    let crash_dir = std::env::current_dir().unwrap().join("crashes_found");

    if crash_dir.exists() {
        std::fs::remove_dir_all(&crash_dir).expect("Failed to delete old crashes dir");
    }

    std::fs::create_dir_all(&crash_dir).expect("Failed to create the crashes dir");


    let mut loop_count = 0;
    let mut crash_count = 0;

    let fm_stdlib = &file_manager_with_stdlib(Path::new(""));
    let parsed_files_stdlib = parse_all(&fm_stdlib);

    loop {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(CONFIG.min_data_length..=CONFIG.max_data_length);
        let vec: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
        let code_generated = generate_code::generate_code(&vec);

        let mut fm = fm_stdlib.clone();
        let mut parsed_files = parsed_files_stdlib.clone();
        
        let parsed = parser::parse_program(&code_generated);
        let file_id = fm.add_file_with_source(&nr_main_path, code_generated.clone());
        parsed_files.insert(file_id.expect("No file id"), parsed);


        let options = noirc_driver::CompileOptions::default();

        let workspace = match resolve_workspace_from_toml(
            &noir_project_dir.join("Nargo.toml"),
            PackageSelection::DefaultOrAll,
            Some(NOIR_ARTIFACT_VERSION_STRING.to_string()),
        ) {
            Ok(w) => w,
            Err(_) => panic!("Can't resolve workspace from toml"),
        };

        let compilation_result = compile_workspace(&fm, &parsed_files, &workspace, &options);

        match compilation_result {
            Ok(_) => {}
            Err(errors) => {
                let mut is_error = false;

                for error in &errors {
                    if error.diagnostic.is_error() && !ignored_error(&error.diagnostic.message){
                        is_error = true;
                    }
                }

                if is_error {
                    crash_count += 1;

                    let crash = format!("crash{}", crash_count);

                    std::fs::create_dir_all(&crash_dir.join(&crash)).expect("Failed to create a crash dir");
                    std::fs::write(&crash_dir.join(&crash).join("code.nr"), &code_generated).expect("Failed to write code");
                    let mut errors_string = String::new();
                    for err in &errors {
                        errors_string = format!("{}\n{}", errors_string, err.diagnostic.message);
                    }
                    std::fs::write(&crash_dir.join(&crash).join("err"), &errors_string).expect("Failed to write err");
                }
            }
        }
        loop_count += 1;
        
        print!("\rLoop Count: {} Crash Count: {}", loop_count, crash_count);
        io::stdout().flush().unwrap();
    }

}