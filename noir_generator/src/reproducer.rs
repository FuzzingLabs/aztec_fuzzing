use std::io;

mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file path as a command line argument.");
        std::process::exit(1);
    }
    let file_path = &args[1];

    let crash_dir = std::env::current_dir().unwrap().join("crashes_found");
    if !crash_dir.exists() {
         std::fs::create_dir_all(&crash_dir).expect("Failed to create the crashes dir");
    }

    let entries = std::fs::read_dir(file_path)?;

    for entry in entries {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_str) = file_name.to_str() {
                    if file_str.starts_with("SIG") {
                        println!("File name : {}", file_str);
                        let data = std::fs::read( std::env::current_dir().unwrap().join(&path)).expect("Could not read file");
                        std::fs::remove_file(std::env::current_dir().unwrap().join(&path))?;
                        let code_generated = generate_code::generate_code(&data);
                        std::fs::write(crash_dir.join(file_str.to_string()), &code_generated).expect("Failed to write main.nr");
                    }
                }
            }
        }

    }

    Ok(())

}

// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     if args.len() < 2 {
//         eprintln!("Please provide a file path as a command line argument.");
//         std::process::exit(1);
//     }
//     let file_path = &args[1];
//     let data = std::fs::read(file_path).expect("Could not read file");

//     let noir_project_dir = std::env::current_dir().unwrap().join("noir_project");
//     let nr_main_path = noir_project_dir.join("src/main.nr");

//     println!("1");
//     let code_generated = generate_code::generate_code(&data);
//     println!(" 2");

//     std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

// }
