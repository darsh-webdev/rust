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
    v[99]; // Attempting to access an element beyond the end of a vector, which will cause a call to panic!
}
