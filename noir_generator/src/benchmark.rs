mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

use std::{path::Path, process::Command};
use noirc_frontend::parser;
use std::time::Instant;
use rand::Rng;
use crate::constants::CONFIG;

fn parse_all(fm: &fm::FileManager) -> noirc_frontend::hir::ParsedFiles {
    let start = Instant::now();
    let ret = fm.as_file_map().all_file_ids().map(|&file_id| (file_id, noirc_frontend::hir::def_map::parse_file(fm, file_id))).collect();
    let run_time = start.elapsed();
    println!("TEST {:?}", run_time);
    ret
}

fn main() {
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");

    let fm_stdlib = &noirc_driver::file_manager_with_stdlib(Path::new(""));
    let parsed_files_stdlib = parse_all(&fm_stdlib);

    loop {
        let mut fm = fm_stdlib.clone();
        let mut parsed_files = parsed_files_stdlib.clone();

        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(CONFIG.min_data_length..=CONFIG.max_data_length);
        let vec: Vec<u8> = (0..size).map(|_| rng.gen::<u8>()).collect();
        let code_generated = generate_code::generate_code(&vec);
        let gen_code = start.elapsed();

        let start = Instant::now();

        std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

        let _ = Command::new("nargo")
                .args(&["compile", "--silence-warnings", "--program-dir", noir_project_dir.to_str().unwrap_or_else(|| panic!("Impossible de convertir le chemin en chaîne UTF-8 valide"))])
                .output();

        let cmd_time = start.elapsed();

        let start = Instant::now();

        let parsed = parser::parse_program(&code_generated);
        let file_id = fm.add_file_with_source(Path::new(""), code_generated);
        parsed_files.insert(file_id.expect("sdfsdfsd"), parsed);

        let mut context = noirc_frontend::hir::Context::new(fm, parsed_files);
        let crate_id = noirc_driver::prepare_crate(&mut context, Path::new(""));
        let options = noirc_driver::CompileOptions::default();

        let _ = noirc_driver::compile_main(&mut context, crate_id, &options, None);

        let run_time = start.elapsed();


        

        println!("GEN CODE = {:?}  et CMD TIME = {:?}  et CLASSIC TIME = {:?}",gen_code, cmd_time, run_time);
    }
}
