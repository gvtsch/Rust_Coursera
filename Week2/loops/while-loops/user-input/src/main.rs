use std::io;

/* This example is a usefur application of 'while' because it allows to continue
asking for user input until user typesa specific word (in this case, "stop") */

fn main() {
    let mut input = String::new();

    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word (type 'stop' to quit): ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        /* io::stdin(): This function returns an instance of Stdin, which represents 
        the standard input stream in Rust.
        .read_line(&mut input): This method reads from the standard input stream until
        it sees a newline character. The text it reads (not including the newline) is
        appended to the string input. This method returns a Result type, which is an 
        enum that can be Ok (containing the number of bytes read) or Err 
        (containing an error information).
        .expect("Failed to read line"): This method is called on the Result returned 
        by read_line. If the Result is Err, it will cause the program to panic and 
        display the message "Failed to read line". If the Result is Ok, it simply 
        returns the number of bytes read, which is then ignored. */
        println!("You typed: {}", input.trim());
    }
}