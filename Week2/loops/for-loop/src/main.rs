fn main() {
    // the for loop is using a range. Note you can use also '(1...10)' or '(1..=10)'
    for i in 1..5 {
        println!("i is {}", i);
    }
    println!("---------------------");

    for i in (1..=5).rev() {
        println!("i is {}", i);
    }
    println!("---------------------");

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("n is {}", n);
    }
    println!("---------------------");
}
