fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get (field);
    // This will not compile
    result.expect("Ops! Sth went wrong").to_string() // result is Nan Option<&str> and not a String. Options are enums of any type
}


fn main() {  
    let chunk = split_string("Hello,World".to_string(), ',', 1);
    println!("Split String: {}", chunk);
}
