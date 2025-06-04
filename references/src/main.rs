fn main() {
    println!("Understanding references in Rust");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // Mutable refrence
    let mut s = String::from("Hello");
    change(&mut s);

    println!("{s}");

    // Dangling references
    // let refrence_to_nothing = dangle(); // Throws an error
    let refrence_to_nothing = no_dangle(); // Throws an error
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the value is not dropped.

fn change(str: &mut String) {
    str.push_str(", world!");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello");  // s is a new String
//     &s // we return a reference to the String, s
// }// Here, s goes out of scope, and is dropped, so its memory goes away. This will throw an error

fn no_dangle() -> String {
    let s = String::from("Hii");
    s
}
