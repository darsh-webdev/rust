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

    // Ownership and functions
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x); // because i32 implements the Copy trait,
                       // x does NOT move into the function,
        println!("{}", x); // so it's okay to use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    // Return Values and Scope
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    // Returning multiple values as tuples
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
