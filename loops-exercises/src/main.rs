use std::io;

fn main() {
    // Temperature Converter
    println!("Fahrenheit to Celsius: Temperature Converter");
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read input");

    let fahrenheit = fahrenheit
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{:.2}°F is {:.2}°C", fahrenheit, celsius);

    // Generate fibonacci number
    println!("Fibonacci number generation");
    println!("Enter the position (n) to get the nth Fibonacci number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: u32 = input
        .trim()
        .parse()
        .expect("Please enter a valid positive integer");

    let fib = fibonacci(input);
    println!("The {}th Fibonacci number is: {}", input, fib);
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}
