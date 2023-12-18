/*There are other conditionals that we con explore in Rust. Like using 'if let' */

fn main() {
    let mut maybe_number: Option<Option<()>> = Some(None);
    // let mut maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("number is {:?}", number);
    } else {
        println!("number is None");
    }
}
