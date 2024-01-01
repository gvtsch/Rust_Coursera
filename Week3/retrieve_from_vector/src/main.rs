use std::vec;

fn get_item(index: usize) {
    // let index = 3; 
    let vec = vec![1, 2, 3, 4, 5];

    let value = vec.get(index).unwrap();

    println!("The value at index {} is: {}", index, value);
}


fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    /* Retrieve a value at a specific index */
    let third_value = vec[2];
    println!("The third value is: {}", third_value);

    /* Retrieve the last value */
    let last_value = vec.last().unwrap(); /* unwrap, because vec.last() is probably an option */
    println!("The last value is: {:?}", last_value);

    /* Retrieve the first value using pattern matching */
    match vec.first() {
        Some(first_value) => println!("The first value is: {}", first_value),
        None => println!("There is no first value. The vctor is empty."),
    }
    
    get_item(3);
}
