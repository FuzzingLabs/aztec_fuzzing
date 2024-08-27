use noir_smith::{generate_code, generate_code_with_seed};
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

/// This program generates code based on either a seed or data from a file.
/// It supports the following command line options:
///
/// Usage:
///     `program_name [-s <seed> | -d <file_path>] [-o <output_path>]`
///
/// Options:
///     - `-s <seed>`: Generate code using the specified seed.
///     - `-d <file_path>`: Generate code using data from the specified file.
///     - `-o <output_path>`: Specify the output file to store the generated code.
///        If not provided, the code will be printed to the console.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 5 {
        eprintln!(
            "Usage: {} [-s <seed> | -d <file_path>] [-o <output_path>]",
            args[0]
        );
        std::process::exit(1);
    }

    let mut mode = None;
    let mut input = None;
    let mut output_path = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-s" => {
                if i + 1 >= args.len() {
                    eprintln!(
                        "Usage: {} [-s <seed> | -d <file_path>] [-o <output_path>]",
                        args[0]
                    );
                    std::process::exit(1);
                }
                mode = Some("seed");
                input = Some(args[i + 1].clone());
                i += 1;
            }
            "-d" => {
                if i + 1 >= args.len() {
                    eprintln!(
                        "Usage: {} [-s <seed> | -d <file_path>] [-o <output_path>]",
                        args[0]
                    );
                    std::process::exit(1);
                }
                mode = Some("file");
                input = Some(args[i + 1].clone());
                i += 1;
            }
            "-o" => {
                if i + 1 >= args.len() {
                    eprintln!(
                        "Usage: {} [-s <seed> | -d <file_path>] [-o <output_path>]",
                        args[0]
                    );
                    std::process::exit(1);
                }
                output_path = Some(args[i + 1].clone());
                i += 1;
            }
            _ => {
                eprintln!(
                    "Usage: {} [-s <seed> | -d <file_path>] [-o <output_path>]",
                    args[0]
                );
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let generated_code = match mode.as_deref() {
        Some("seed") => {
            let seed: u64 = input
                .expect("Seed argument is missing")
                .parse()
                .expect("Please provide a valid u64 for the seed");
            generate_code_with_seed(seed)
        }
        Some("file") => {
            let file_path = input.expect("File path argument is missing");
            let mut file = File::open(&file_path).expect("Failed to open the input file");
            let mut data = Vec::new();
            file.read_to_end(&mut data)
                .expect("Failed to read the input file");
            generate_code(&data)
        }
        _ => {
            let seed: u64 = rand::thread_rng().gen();
            generate_code_with_seed(seed)
        }
    };

    if let Some(output_path) = output_path {
        match File::create(&output_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(generated_code.as_bytes()) {
                    eprintln!("Failed to write to file: {}", e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to create file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("{}", generated_code);
    }
}
