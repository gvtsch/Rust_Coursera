fn main() {
    let proceed = true;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Stopped");
    }

    let height = 190;
    if height > 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }
}
