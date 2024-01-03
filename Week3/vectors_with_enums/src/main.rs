use std::vec;

enum Shape {
    Square(f64),
    Circle(f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Square(side) => side * side,
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

fn largest_shape(shapes: &[Shape]) -> &Shape {
    shapes.iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap()).unwrap()
}

fn main() {
    let shapes = vec![
        Shape::Square(3.0),
        Shape::Circle(5.0),
        Shape::Circle(4.0),
        Shape::Triangle(3.0, 4.0),
    ];

    let largest = largest_shape(&shapes);
    match largest {
        Shape::Square(side) => println!("Largest shape is a square with side length {}", side),
        Shape::Circle(radius) => println!("Largest shape is a circle with radius {}", radius),
        Shape::Triangle(base, height) => println!("Largest shape is a triangle with base {} and height {}", base, height),
    }
}