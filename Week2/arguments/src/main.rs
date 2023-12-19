use std::io;
fn average(numbers: &[i32], num_elements: usize) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result / num_elements as i32
}

fn main() {
    let mut input = String::new();

    println!("Enter the number of elements:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_elements: usize = input.trim().parse().expect("Invalid input");

    let mut numbers = Vec::new();
    for _ in 0..num_elements {
        input.clear();
        println!("Enter a number:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let number: i32 = input.trim().parse().expect("Invalid input");
        numbers.push(number);
    }

    let result = average(&numbers, num_elements);
    println!("Sum of numbers: {}", result);
}

/*  */