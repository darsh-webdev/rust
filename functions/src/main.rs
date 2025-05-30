fn main() {
    println!("Exploring functions in Rust");

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let z = five();
    println!("The value of z is: {z}");

    let a = plus_one(9);
    println!("The value of a is: {a}");

    another_function(7);
    print_labeled_measurement(5, 'm');
}

// Function with single parameter
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// Function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Function with return value
fn five() -> i32 {
    5
}

// Function with parameter and return value
fn plus_one(x: i32) -> i32 {
    x + 1
}
