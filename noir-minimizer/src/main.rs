use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];

    // Open the file
    let mut file = File::open(file_path)?;

    // Create a vector to hold the file contents
    let mut contents = Vec::new();

    // Read the file contents into the vector
    file.read_to_end(&mut contents)?;

    // Convert the contents to a string and print it
    let contents_str = String::from_utf8_lossy(&contents);
    println!("{}", contents_str);

    Ok(())
}
