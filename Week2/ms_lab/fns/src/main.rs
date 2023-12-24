fn divide_by_5(num: i32) -> i32 {
    num / 5
}

fn goodbye(message: &str) -> bool {
    println!("\n{}", message);
    return true
}

fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
    
    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
}