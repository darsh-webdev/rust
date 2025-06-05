fn main() {
    println!("Understanding slices in Rust");
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    // String slices
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let str = String::from("world");

    // If you want to start at index 0, you can drop the starting index
    let slice = &str[..2];

    // If you want the slice till the last byte, you can drop the ending index
    let slice = &str[3..];

    // If you want to slice the entire string, drop both the starting and ending index
    let slice = &str[..];

    let word = first_word_using_slice(&s);

    println!("First word is: {word}");

    // Array slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_using_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
