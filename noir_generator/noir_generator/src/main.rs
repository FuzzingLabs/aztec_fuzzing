mod random;
mod generate_function;
mod instructions;
mod variables;
mod statements;
mod constants;

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
extern crate honggfuzz;

fn fuzz_by_command_line(data: Option<&[u8]>) -> Option<String> {
    random::initialize_rng(data);
    
    let mut code_generated = String::new();
    for _ in 0..random::gen_range(0, constants::NB_MAX_FUNCTION) {
        code_generated = format!("{}{}\n", code_generated, generate_function::generate_function(random::gen_name()));
    }
    code_generated = format!("{}{}", code_generated, generate_function::generate_function("main".to_string()));

    let nr_file_path = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr";
    std::fs::write(nr_file_path, &code_generated).expect("Failed to write temp file");

    let compilation_result = Command::new("/home/afredefon/.nargo/bin/nargo")
        .args(&["compile", "--program-dir", "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test", "--package", "test"])
        .output();

    match compilation_result {
        Ok(output) => {
            if output.status.success() {
                None
            } else {
                Some(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => {
            eprintln!("Error executing compilation command: {}", e);
            None
        }
    }
}

fn ignored_error(err: &String) -> bool{
    err.contains("attempt to divide by zero")
}


fn main() {
    let num_threads = 4;

    // Utilisez Arc (Atomic Reference Counter) pour partager les données entre les threads
    let mut loop_count = 1;
    let mut crash_count = 1;

    let _ = std::fs::remove_dir_all("/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/crash_found/");
    std::fs::create_dir("/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/crash_found/");

    loop {
        match fuzz_by_command_line(None) {
            Some(err) => {
                if !ignored_error(&err) {
                    let src = "/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/testNoir/test/src/main.nr";
                    let dest = format!("/home/afredefon/FuzzingLabs/aztec_fuzzing/noir_generator/noir_generator/crash_found/crash{}.nr", crash_count.to_string());
                    let _ = std::fs::copy(src, dest);
    
                    println!("CRASH N°{} FOUND AFTER {} LOOP", crash_count, loop_count);
                    println!("{}", err);
                    crash_count += 1;
                }
            }
            None => {}
        }
        loop_count += 1;
    }

    // honggfuzz::fuzz!(|data: &[u8]| {
    //     if let Some(err) = fuzz_by_command_line(Some(data)) {
    //         if !ignored_error(&err) {
    //             println!("CMD Error: {}", err);
    //         }
    //     }
    // });

}
