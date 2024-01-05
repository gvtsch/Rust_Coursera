use std::env;
use std::io;

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

/*
fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{} GB", gb as f64 / 1000.0),
        FileSize::Terabytes(gb) => format!("{} GB", gb as f64 / 1000.0),
    }
}
*/

impl FileSize {
    // Implement a method called format_size() that takes a value and a unit as arguments and returns a String.
    // The method should return a String that should contain several values and units.
    // For example, if the value is 100 and the unit is kilobytes, the method should return "0.1 mb" "0.0001 gb" asf.

    fn format_size(size: u64, unit: &str) -> String {
        let filesize = match unit {
            "bytes" => FileSize::Bytes(size),
            "kb" => FileSize::Kilobytes(size),
            "mb" => FileSize::Megabytes(size),
            "gb" => FileSize::Gigabytes(size),
            "tb" => FileSize::Terabytes(size),
            _ => panic!("Invalid unit"),
        };

        match filesize {
            FileSize::Bytes(bytes) => format!("{} kb\n {} MB\n{} GB\n{} TB", bytes as f64 * 1_000.0, bytes as f64 * 1_000_000.0, bytes as f64 * 1_000_000_000.0, bytes as f64 / 1_000_000_000_000.0),
            FileSize::Kilobytes(kb) => format!("{} bytes\n {} MB\n{} GB\n{} TB", kb as f64 * 1_000.0, kb as f64 / 1000.0, kb as f64 / 1_000_000.0, kb as f64 / 1_000_000_000.0),
            FileSize::Megabytes(mb) => format!("{} bytes\n {} kb\n{} GB\n{} TB", mb as f64 * 1_000_000.0, mb as f64 * 1_000.0, mb as f64 / 1_000.0, mb as f64 / 1_000_000.0),
            FileSize::Gigabytes(gb) => format!("{} bytes\n {} kb\n{} MB\n{} TB", gb as f64 * 1_000_000_000.0, gb as f64 * 1_000_000.0, gb as f64 / 1_000.0, gb as f64 / 1_000.0),
            FileSize::Terabytes(tb) => format!("{} bytes\n {} kb\n{} MB\n{} GB", tb as f64 * 1_000_000_000_000.0, tb as f64 * 1_000_000_000.0, tb as f64 * 1_000_000.0, tb as f64 * 1_000.0),
        }
    }
}


    fn main() {
        let mut input = String::new();

        println!("Enter size and unit (e.g., 100 kilobytes):");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        /* 
        io::stdin(): This function returns a handle to the standard input stream. It allows you to read input from the user.
        .read_line(&mut input): This method is called on the standard input handle returned by io::stdin(). 
            It reads a line of input from the user and stores it in the input variable. The &mut input part indicates that 
            input is a mutable reference, allowing the method to modify its value.
        .expect("Failed to read input"): This is an error handling method that is chained to the read_line() method. It 
            expects the read_line() method to return a Result type, which represents the result of an operation that can either 
            be successful (Ok) or contain an error (Err). If an error occurs during the reading process, the expect() method 
            will panic and display the specified error message, "Failed to read input". 
        */

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input format");
            return;
        }

        let size: u64 = match parts[0].parse() {
            Ok(size) => size,
            Err(_) => {
                println!("Invalid size input");
                return;
            }
        };

        let unit = match parts[1] {
            "bytes" | "kb" | "mb" | "gb" | "tb" => parts[1],
            _ => {
                println!("Invalid unit input: {:?}", parts[1]);
                return;
            }
        };

        // println!("Size: {:?} \nUnit: {:?}", size, unit);

        let formatted_size = FileSize::format_size(size, unit);
        println!("Formatted Size:\n{}", formatted_size);

}