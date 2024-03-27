mod random;
mod generate_code;
mod instructions;
mod variables;
mod statements;
mod constants;
mod functions;

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


    let code_generated = generate_code::generate_code(&data);

    std::fs::write(&nr_main_path, &code_generated).expect("Failed to write main.nr");

}
