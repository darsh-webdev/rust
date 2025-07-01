use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
fn main() {
    println!("Understanding Error Handling in Rust");
    // Note: Rust groups errors into two major categories: recoverable and unrecoverable errors.
    // Most languages don’t distinguish between these two kinds of errors and handle both in the same way,
    // using mechanisms such as exceptions. Rust doesn’t have exceptions.
    // Instead, it has the type Result<T, E> for recoverable errors and the panic!
    // macro that stops execution when the program encounters an unrecoverable error.

    // Part 1 - Unrecoverable Errors with panic!
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    // v[99]; // Attempting to access an element beyond the end of a vector, which will cause a call to panic!

    // Part 2 - Recoverable Errors with Result
    let greeting_file_result = File::open("hello.txt"); // Let’s call a function that returns a Result value because the function could fail

    //  Using a match expression to handle the Result variants that might be returned
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // Handling different kinds of errors in different ways
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // Shortcuts for Panic on Error: unwrap and expect
    let greeting_file = File::open("hello.txt").unwrap(); // If we run this code without a hello.txt file, we’ll see an error message from the panic! call that the unwrap method makes

    // Expect method lets us also choose the panic! error message.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating Errors
    let user = read_username_from_file();
}

// A function that returns errors to the calling code using match
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
