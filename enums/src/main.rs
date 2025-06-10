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
}
