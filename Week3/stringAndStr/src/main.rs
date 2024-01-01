fn print_str(s: &str) {
    let mut new_string = s.to_string();
    new_string.push_str("!\n");

    let mut new_new_string = format!("{}?", s);
    println!("{}{}", new_string, new_new_string);
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s: &str= "hello world!";
    print_str(s);

    /* String is growable cnd mutable whereas str is not.
    String is owned by the code that creates it */
    let mut salutation = String::from("hello");
    print_string(salutation);
}

/* In Rust, String and str are two different types used to handle string data, but they serve 
different purposes:

String: This is a growable, heap-allocated data structure. You would use String when you need 
a mutable string where the size is unknown at compile time, or when you want to own the string 
data. It's similar to strings in many other programming languages.

str: This is a string slice, which is a reference to some string data. It's typically seen in 
its borrowed form, &str. You would use str when you want to refer to a string without taking 
ownership, such as when you're reading string data or when you're passing a string to a 
function and don't need to mutate it. */