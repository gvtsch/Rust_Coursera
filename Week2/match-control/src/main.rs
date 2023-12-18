use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new(); // create a mutable variable 'name' of type String
    io::stdin().read_line(&mut name).expect("Failed to read line"); // read line from stdin and store in 'name'

    // use match expression to pattern match against varialbe 'name'
    match name.trim() { // is like case in other languages
        // if 'name' is "Hello" then execute this block
        "Hello" => println!("Hello there!"),
        // if 'name' is "Goodbye" then execute this block
        "Goodbye" => println!("Goodbye!"),
        // if 'name' is anything else then execute this block
        _ => println!("What? Goodbye!"),
    }
}
