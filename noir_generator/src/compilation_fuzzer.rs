#[macro_use]
extern crate honggfuzz;

mod constants;
mod generate_function;
mod instructions;
mod random;
mod statements;
mod variables;

use chrono::{DateTime, Utc};
use core::time;
use nargo_cli;
use noirc_frontend::parser;
use std::io::{self, Write};
use std::process::Command;

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
    let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
    let crash_dir = std::env::current_dir().unwrap().join("crashes_found");
    let nr_main_path = noir_project_dir.join("src/main.nr");
    if crash_dir.exists() {
        std::fs::remove_dir_all(&crash_dir).expect("Failed to delete old crashes dir");
    }

    std::fs::create_dir_all(&crash_dir).expect("Failed to create the crashes dir");
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() != 8 {
                return;
            }
            random::initialize_rng(Some(data));
            let code_generated = generation();
            std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");
            match nargo_cli::fuzzinglabs_run() {
                Ok(_) => {}
                Err(err) => {
                    /*                     if ignored_error(&err) {
                        return;
                    } */
                    let now: DateTime<Utc> = Utc::now();
                    let timestamp_str = now.to_rfc3339();
                    std::fs::write(&crash_dir.join(timestamp_str), &code_generated)
                        .expect("Failed to write err");
                    panic!("Error running program: {:?}", err);
                }
            }

            /*             if parse_error.len() > 0 {
                let now: DateTime<Utc> = Utc::now();
                let timestamp_str = now.to_rfc3339();
                std::fs::write(&crash_dir.join(timestamp_str), &seed).expect("Failed to write err");
                panic!(
                    "Error parsing program: {:?} with data {:?}",
                    parse_error, seed
                );
            } */
        });
    }
}
