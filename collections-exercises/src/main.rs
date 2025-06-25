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
}
