fn main() {
    // Creating a new empty string
    let mut s = String::new();

    let data = "initial contents";
    // Using the to_string method to create a String from a string literal
    let s = data.to_string();
    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    // Using the String::from function to create a String from a string literal
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded so we can store string in different languages
    let hello = String::from("Hello");
    let hello = String::from("नमस्ते");

    // Updating a string
    println!("\nUsing push_str method::::");
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("The string after appending is: {s}");

    // Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // Note: The push method takes a single character as a parameter and adds it to the String
    println!("\nUsing push method::::");
    let mut s = String::from("lo");
    s.push('l');
    println!("String after pushing a single character is: {s}");

    // Concatenation with the + Operator or the format! Macro
    println!("\nUsing + operator::::");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("String s3 is: {s3}");

    // Gets difficult to concate multiple strings using + operator
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // s = tic-tac-toe

    // To make is less complicated, we can instead use the format! macro
    println!("\nUsing format! macro::::");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is: {s}"); // Output: s is: tic-tac-toe
}
