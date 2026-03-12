// Original Function to get Area 
/*
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
        );
}

fn area (width: u32, height: u32) -> u32{
    width * height
}*/


// Now Refactored w/Tuples
/*
fn main (){
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area(rect1)
    );
}

fn area (dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
} */

// Now Refactoring w/Structs
 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
//using its width and height fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity
