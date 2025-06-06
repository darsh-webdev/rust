struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Using structs in Rust");

    // Creating an instance of the User struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Changing the value in the email field of a User instance
    user1.email = String::from("anotheremail@example.com");

    // Creating a new User instance using all but one of the values from user1
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Using struct update syntax to set a new email value for a User instance but to use the rest of the values from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Defining Tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
