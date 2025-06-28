use std::collections::HashMap;
use std::io;
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

    // Problem Statement 3: Using a hash map and vectors, create a text interface
    // to allow a user to add employee names to a department in a company;
    // for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
    // a list of all people in a department or all people in the company by department, sorted alphabetically.

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    println!("\n\nSolution for Problem Statement 3:::::");
    loop {
        println!("\nEnter command (or 'quit' to exit):");
        println!("- Add [Name] to [Department]");
        println!("- List [Department]");
        println!("- List all");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "quit" {
            break;
        }

        if input.starts_with("Add ") && input.contains(" to ") {
            let parts: Vec<&str> = input[4..].splitn(2, " to ").collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let department = parts[1].to_string();

                departments
                    .entry(department.clone())
                    .or_insert_with(Vec::new)
                    .push(name.clone());

                println!("Added {} to {}", name, department);
            }
        } else if input == "List all" {
            let mut dept_names: Vec<&String> = departments.keys().collect();
            dept_names.sort();

            for dept in dept_names {
                let mut employees = departments[dept].clone();
                employees.sort();
                println!("{}:", dept);
                for employee in employees {
                    println!("  {}", employee);
                }
            }
        } else if input.starts_with("List ") {
            let department = &input[5..];
            if let Some(employees) = departments.get(department) {
                let mut sorted_employees = employees.clone();
                sorted_employees.sort();
                println!("Employees in {}:", department);
                for employee in sorted_employees {
                    println!("  {}", employee);
                }
            } else {
                println!("Department '{}' not found", department);
            }
        } else {
            println!("Invalid command");
        }
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
