use std::collections::HashMap;

fn main() {
    // Creating a new hash map
    let mut scores = HashMap::new();
    // Adding key and values using insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:#?}", scores);

    // Note: Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.
    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // This above code handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
    println!("Team {team_name} score is: {score}");

    // Iterating over each key-value pair using for loop
    for (key, value) in scores {
        println!("{key}: {value}");
    }

    // Hash maps and Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point as they are now owned by the hash map.

    // Updating a hash map
    println!("\nUpdating by overwriting a value::::");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("Updated Scores: {scores:?}"); // It will print {"Blue": 25}. The original value of 10 has been overwritten.

    println!("\nUpdating by adding a key-value only if a key isnt present::::");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Using the entry method to only insert if the key does not already have a value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("Updated Scores: {scores:?}"); // It will print {"Blue": 10, "Yellow": 50}.

    println!("\nUpdating a value based on the old value::::");
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // Counting occurrences of words using a hash map that stores words and counts
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Occurences of words are: {map:?}");
}
