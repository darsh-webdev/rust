// Defining an enum whose variants each store different amounts and types of values
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Defining methods on enum
impl Message {
    fn call(&self) {
        println!("Printing using call method")
    }
}

fn main() {
    println!("Understanding enums in Rust");

    let m = Message::Write(String::from("Hello World!"));
    m.call();

    // Usage of the "Option" enum
    let some_number = Some(5);
    let some_char = Some('d');

    let absent_number: Option<i32> = None;

    // The below commented code will throw an error,
    // Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}
