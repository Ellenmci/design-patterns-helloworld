use std::any::type_name;

fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    let count = 10.0;
    println!("count is {}", count);
    println!("The inferred type of count is: {}", type_of(&count));
}

/* exercise 3
use std::io;

fn main() {
    let mut name = String::new();
    let mut color = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter your favorite color: ");
    io::stdin().read_line(&mut color).expect("Failed to read input");

    // Trim the newline character from input
    let name = name.trim();
    let color = color.trim();

    println!("Hello, {}! Your favorite color is {}.", name, color);
}
*/

/* exercise 4
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter temperature in Celsius: ");
    io::stdin().read_line(&mut input).unwrap();

    let celsius: f64 = input.trim().parse().expect("Please enter a valid number");

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C is {:.2}°F", celsius, fahrenheit);
}
*/

/* exercise 5
use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter the length of the rectangle: ");
    io::stdin().read_line(&mut input).unwrap();
    let length: f64 = input.trim().parse().expect("Invalid number");

    input.clear();
    println!("Enter the width of the rectangle: ");
    io::stdin().read_line(&mut input).unwrap();
    let width: f64 = input.trim().parse().expect("Invalid number");

    println!("Area: {:.2}", length * width);
    println!("Perimeter: {:.2}", 2.0 * (length + width));
}


solution 2

use std::io;

fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Invalid number")
}

fn main() {
    let length = read_number("Enter the length of the rectangle: ");
    let width = read_number("Enter the width of the rectangle: ");

    println!("Area: {:.2}", length * width);
    println!("Perimeter: {:.2}", 2.0 * (length + width));
}
*/
