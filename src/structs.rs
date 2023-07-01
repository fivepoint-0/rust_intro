#[derive(Debug)] // This allows for printing of the object without needing to implement any needed functions

struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height
}

pub fn rectangle_area_example() {
    let rectangle = Rectangle {
        width: 50,
        height: 50
    };

    println!("Area of the rectangle is {}", area(&rectangle));

    println!("Width of rectangle: {}", rectangle.width);
    println!("Height of rectangle: {}", rectangle.height);

    println!("Rectangle object: {:?}", rectangle); // The use of ":?" allows us to print an object that doesnt implement some other rust functions
}