fn main() {
    println!("Understanding concise control flow with if let and let else in Rust");

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Re-write above code in a shorter way using if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // Understanding let..else
    let user_inputs = ["42", "hello", "123", "world", "0"];
    println!("\n=== Using traditional match ===");
    for input in &user_inputs {
        parse_with_match(input);
    }

    println!("\n=== Using let else ===");
    for input in &user_inputs {
        parse_with_let_else(input);
    }
}

// Traditional approach using match
fn parse_with_match(input: &str) {
    let number = match input.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("'{}' is not a valid number, skipping...", input);
            return; // Early return on error
        }
    };

    // Continue processing the valid number
    let doubled = number * 2;
    println!("Number: {}, Doubled: {}", number, doubled);
}

// Simplified approach using let else
fn parse_with_let_else(input: &str) {
    // If parsing fails, execute the else block and return early
    let Ok(number) = input.parse::<i32>() else {
        println!("'{}' is not a valid number, skipping...", input);
        return;
    };

    // Continue processing the valid number
    let doubled = number * 2;
    println!("Number: {}, Doubled: {}", number, doubled);
}
