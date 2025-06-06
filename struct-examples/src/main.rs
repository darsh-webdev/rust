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
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
