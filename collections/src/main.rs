fn main() {
    // Create a new empty vector to hold values of type i32
    let v: Vec<i32> = Vec::new();

    // Create a new vector with values
    let v = vec![1, 2, 3];

    // Add values to vector using push
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("The vector after pushing values is {:?}", v);

    // Reading elements from vector
    let v = vec![1, 2, 3, 4, 5];

    // Using indexing
    println!("\nUsing Indexing:::::");
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Using get method
    println!("\nUsing Get method:::::");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterating over the values in a vector and printing using for loop
    println!("\nUsing for loop:::::");
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Iterating over mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
