use std::collections::HashMap;
fn main() {
    // Problem Statement 1: Given a list of integers, use a vector
    // and return the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let numbers = vec![4, 1, 7, 2, 7, 5, 7, 3, 1, 1];
    // Calculate median
    let mut sorted = numbers.clone();
    sorted.sort();
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) as f64 / 2.0
    } else {
        sorted[sorted.len() / 2] as f64
    };

    // Calculate mode using HashMap
    let mut count = HashMap::new();
    for num in &numbers {
        *count.entry(num).or_insert(0) += 1;
    }

    let max_count = count.values().max().unwrap();
    let mode: Vec<i32> = count
        .iter()
        .filter(|(_, freq)| **freq == *max_count)
        .map(|(num, _)| **num)
        .collect();

    println!("\nSolution for Problem Statement 1:::::");
    println!("Numbers: {:?}", numbers);
    println!("Median: {}", median);
    println!("Mode: {:?}", mode);

    // Problem Statement 2: Convert strings to pig latin. The first consonant of each word
    // is moved to the end of the word and ay is added, so first becomes irst-fay.
    // Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
    // Keep in mind the details about UTF-8 encoding!

    let text = "hello world first apple";
    println!("\nSolution for Problem Statement 2:::::");
    println!("Text:\n{text}");
    println!("Pig Latin Text:");
    for word in text.split_whitespace() {
        print!("{} ", pig_latin(word));
    }
}

// Function for text to pig latin
fn pig_latin(word: &str) -> String {
    let vowels = "aeiouAEIOU";

    if let Some(first_char) = word.chars().next() {
        if vowels.contains(first_char) {
            format!("{}-hay", word)
        } else {
            let rest: String = word.chars().skip(1).collect();
            format!("{}-{}ay", rest, first_char)
        }
    } else {
        word.to_string()
    }
}
