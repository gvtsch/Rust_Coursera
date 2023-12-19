fn own_vec(mut vector: &Vec<i32>) -> Vec<i32> {
    // create a new vector
    let mut new_vector = Vec::new();
    new_vector.push(10);
    new_vector
    /*return 
    vector.push(10);
    println!("my_vec: {:?}", vector);*/
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: &String) {
    println!("my_str: {}", s);
}

/* Borrowing is tho mechanism by which Rust allows you to lend of a variable
to a function or another part of your program without giving up ownership.
When you borrow a variablev, you're essentially saying:
" I want to use this variable, but I promise I won't modify it". */

fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let my_str = "Hello, World!".to_string();

    // this compiles no problem
    own_integer(my_int);
    println!("my_int: {}", my_int);

    own_string(&my_str); // take ownership of my_str
    // this is using my_string which has also moved and is invalid
    println!("my_str: {:?}", my_str); // this will not compile if my_str is not borrowed (&)

    let new_vector = own_vec(&my_vec); // borrow my_vec
    // but this is using my_vec which was borrowed and yet is now invalid
    println!("my_vec: {:?}", new_vector); 
}

