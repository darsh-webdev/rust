// Adding the attribute to derive the Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Calculating area of Rectangle using tuples
    let rect = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect));

    // Refactoring with use of struct
    let rect1 = Rectangle {
        width: 20,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_struct(&rect1)
    );

    println!("rect1 is {rect1:#?}");

    // Print out a value using the Debug format using the dbg! macro
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
