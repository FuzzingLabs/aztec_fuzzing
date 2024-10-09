use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::process::Command;

// Constants for file paths and directories
const TARGET_FILE_PATH: &str = "./noirbin/src/main.nr";
const MINIMIZED_OUTPUT_DIR: &str = "./minimized_output/";
const FINAL_OUTPUT_PATH: &str = "minimized_output.nr";

fn main() -> io::Result<()> {
    println!("[+] Minimizing the crash...");

    // Get the file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];

    // Read the contents of the file
    let contents = read_file_contents(file_path)?;
    let mut contents_str = String::from_utf8_lossy(&contents).to_string();

    // Ensure the minimized output directory exists
    fs::create_dir_all(MINIMIZED_OUTPUT_DIR)?;

    // Initialize variables for minimization
    let mut previous_error = String::new();
    let mut file_counter = 1;
    let mut minimized = false;

    // Main minimization loop
    let mut i = 0;
    while i < contents_str.len() {
        // Execute the command with the current contents
        if execute_command(&contents_str)? {
            println!("Command executed successfully.");
            // Exit the loop if the command succeeds
            break;
        } else {
            // Get the error message from the command output
            let error_message = get_error_message()?;

            // If it's the first iteration or the error message is the same, attempt minimization
            if previous_error.is_empty() || error_message == previous_error {
                // Try removing a range of characters starting at position `i`
                let new_contents = attempt_minimization(&contents_str, i, 5)?;

                // Check if the error remains the same with the modified contents
                if error_message == get_error_message()? {
                    // If error message is the same, keep the change (minimized)
                    contents_str = new_contents;
                    minimized = true;

                    // Save intermediate minimized content to a numbered file in ./minimized_output/
                    save_intermediate_file(&contents_str, file_counter)?;
                    file_counter += 1;

                    // No need to increment `i`, re-check the same position in case more removals can be done
                    continue;
                }
            }

            // Store the previous error message for comparison
            previous_error = error_message;
        }

        // Move to the next character
        i += 1;
    }

    // Save the final minimized output if any minimization was done
    if minimized {
        save_final_output(&contents_str)?;
        println!(
            "Final minimized output saved to {}",
            format!("{}/{}", MINIMIZED_OUTPUT_DIR, FINAL_OUTPUT_PATH)
        );
    } else {
        println!("No minimization was done.");
    }

    Ok(())
}

// Reads the contents of the file
fn read_file_contents(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

// Executes the `nargo execute` command with the current contents
fn execute_command(contents: &str) -> io::Result<bool> {
    fs::write(TARGET_FILE_PATH, contents.as_bytes())?;
    let output = Command::new("nargo")
        .arg("execute")
        .current_dir("./noirbin")
        .output()
        .expect("Failed to execute command");
    Ok(output.status.success())
}

// Retrieves the error message from the command output
fn get_error_message() -> io::Result<String> {
    let output = Command::new("nargo")
        .arg("execute")
        .current_dir("./noirbin")
        .output()
        .expect("Failed to execute command");
    Ok(String::from_utf8_lossy(&output.stderr).to_string())
}

// Attempts to minimize the content by removing a range of characters starting at the specified index
fn attempt_minimization(contents: &str, index: usize, range_length: usize) -> io::Result<String> {
    let mut new_contents = contents.to_string();
    new_contents.replace_range(index..index + range_length, "");
    fs::write(TARGET_FILE_PATH, new_contents.as_bytes())?;
    Ok(new_contents)
}

// Saves intermediate minimized content to a numbered file in ./minimized_output/
fn save_intermediate_file(contents: &str, counter: usize) -> io::Result<()> {
    let intermediate_file_path = format!("{}{}.nr", MINIMIZED_OUTPUT_DIR, counter);
    fs::write(&intermediate_file_path, contents.as_bytes())?;
    println!("Minimized content saved to {}", intermediate_file_path);
    Ok(())
}

// Saves the final minimized output to ./minimized_output/minimized_output.nr
fn save_final_output(contents: &str) -> io::Result<()> {
    let final_output_path = format!("{}/{}", MINIMIZED_OUTPUT_DIR, FINAL_OUTPUT_PATH);
    fs::write(&final_output_path, contents.as_bytes())?;
    Ok(())
}
