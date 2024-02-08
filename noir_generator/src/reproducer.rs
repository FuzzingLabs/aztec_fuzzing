mod constants;
mod generate_function;
mod instructions;
mod random;
mod statements;
mod variables;

use std::io::{self, Write};
use std::process::Command;

use chrono::{DateTime, Utc};
use noirc_frontend::parser;

fn ignored_error(err: &String) -> bool {
    err.contains("attempt to divide by zero")
        || err.contains("Comparisons are invalid on Field types.")
}

fn generation() -> String {
    let mut code_generated = String::new();
    for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
        code_generated = format!(
            "{}{}\n",
            code_generated,
            generate_function::generate_function(random::gen_name())
        );
    }
    code_generated = format!(
        "{}{}",
        code_generated,
        generate_function::generate_function("main".to_string())
    );
    code_generated
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path as a command line argument.");
        std::process::exit(1);
    }
    let file_path = &args[1];
    let data = std::fs::read(file_path).expect("Could not read file");
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let nr_main_path = noir_project_dir.join("src/main.nr");
    let crash_dir = std::env::current_dir().unwrap().join("crashes_found");
    let nr_main_path = noir_project_dir.join("src/main.nr");
    if crash_dir.exists() {
        std::fs::remove_dir_all(&crash_dir).expect("Failed to delete old crashes dir");
    }
    random::initialize_rng(Some(&data));
    let code_generated = generation();
    std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");
    match nargo_cli::fuzzinglabs_run() {
        Ok(_) => {}
        Err(err) => {
            /*                     if ignored_error(&err) {
                return;
            } */
            panic!("Error running program: {:?}", err);
        }
    }
}
