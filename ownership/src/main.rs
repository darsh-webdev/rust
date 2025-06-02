fn main() {
    println!("Understanding ownership in Rust");

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Using the String type
    {
        let mut s = String::from("hello"); // s is valid from this point forward
        s.push_str(", world!"); // push_str() appends a literal to a string
        println!("{s}");
    } // this scope is now over, and s is no longer valid

    // Memory allocation
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    // Scope and Assignment
    // Note:  When you assign a completely new value to an existing variable,
    // Rust will call drop and free the original value’s memory immediately.
    let mut s = String::from("Hello");
    let s = String::from("Ahoy");

    println!("{s}, world!"); // This will print "Ahoy, world!"

    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
