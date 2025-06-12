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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
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

    // Testing plus_one function
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of var six is {:?}", six); // This will print Some(6)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value, else return "None"
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
