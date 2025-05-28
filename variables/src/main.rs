fn main() {
    println!("Exploring variables in Rust!");
    // Declaring variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Value of constant is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("Value of y in inner scope is: {y}");
    }

    println!("Value of y is: {y}");
}
