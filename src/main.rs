use std::io::{self, Write};

fn main() {
    println!("Welcome to the rectangle area calculator!");

    // Get the length of the rectangle from the user
    print!("Enter the length of the rectangle: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let length: f64 = input.trim().parse().unwrap();

    // Get the width of the rectangle from the user
    print!("Enter the width of the rectangle: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let width: f64 = input.trim().parse().unwrap();

    // Calculate the area of the rectangle
    let area = length * width;

    // Display the area to the user
    println!("The area of the rectangle is {} square units.", area);
}
