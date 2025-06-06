fn main() {
    println!("Understanding control flow in Rust");

    let num = 3;
    // If statement
    if num < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // else if statement
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // loop with return value
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!");

    // For loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // For loop using range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}
