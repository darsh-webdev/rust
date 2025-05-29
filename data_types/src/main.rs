fn main() {
    println!("Understanding data types in Rust");

    // Floating-point Type
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // <--------------------- Numeric Operations --------------------->
    // addition
    let sum = 5 + 10;

    // subtraction
    let diff = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    // Boolean Type
    let t = true;
    let f: bool = false; // explicit type annotation

    // Character Type (char literals require single quotes)
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // <--------------------- Compound Types --------------------->
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Getting individual values of a tuple
    let (x, y, z) = tup; // Destructuring

    println!("The value of y is: {y}"); // This will print 6.4.

    // Array type
    let arr = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Declaring array with type of element and size
    let a = [3; 5]; // This will contain 5 elements all the value as 3. Same as let a = [3, 3, 3, 3, 3];

    // Accessing array elements
    let first = arr[0]; // gets value 1
    let second = arr[1]; // gets value 2

    // Note: If you try to access an element past the end of the array, such as 10, it will cause a runtime error (index out of bounds)
}
