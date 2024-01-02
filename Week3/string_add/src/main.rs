use std::vec;

fn main() {
    let mut v = vec![1, 2, 3];	
    v.push(4);
    println!("v = {:?}", v);   

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("v = {:?}", v);
    /* println!("more_numbers = {:?}", more_numbers);
    when you call v.extend(more_numbers);, the ownership of more_numbers is moved to the extend 
    method. After this line, more_numbers is no longer valid, and trying to use it with 
    println!("more_numbers = {:?}", more_numbers); results in a compile error. */
    
    // append adds the given vector to the vector, requires the given vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("v = {:?}", v);
    println!("other_numbers = {:?}", other_numbers);

    // insert items at a given index
    v.insert(0, 0);
    println!("v = {:?}", v);

    // A function taking a vector and a value as argument
    fn add_to_vec(v: &mut Vec<i32>, value: i32) {
        v.insert(0, value);
        v.push(value);
    }

    add_to_vec(&mut v, 11);
    println!("v = {:?}", v);

    
    fn add_vec_to_vec(v: &mut Vec<i32>, values: &[i32]) {
        for value in values {
            v.push(*value);
        }
        
    }
    add_vec_to_vec(&mut v, &vec![22,23,24]);
    println!("v = {:?}", v);


}
