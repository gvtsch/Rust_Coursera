use std::env; // Import the env module to access command-line arguments
use std::fs::File; // Import the File struct from the std::fs module
use std::io::{BufRead, BufReader}; // Import the BufRead and BufReader traits from the std::io module

fn main() {
    let args: Vec<String> = env::args().collect(); // Get the command-line arguments
    if args.len() < 2 {
        panic!("Please provide a file path as a command-line argument"); // If no file path is provided, panic with an error message
    }
    let file_path = &args[1]; // Get the file path from the command-line arguments

    let file = File::open(file_path);
    let file = match file {
        Ok(file) => file, // If the file was successfully opened, assign it to the 'file' variable
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error) // If the file was not found, panic with an error message
                }
                _ => {
                    panic!("Error opening file: {}", error) // If there was an error opening the file, panic with an error message
                }
            }
        }
    };
    
    let reader = BufReader::new(file); // Create a BufReader instance to read the contents of the file
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line), // If a line was successfully read, print it
            Err(error) => {
                panic!("Error reading line: {}", error) // If there was an error reading a line, panic with an error message
            }
        }
    }
}