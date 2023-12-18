fn main() {
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        println!("i is {}", i);
        if i == 7 {
            break;
        }
    }    
}
