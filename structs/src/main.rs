//Like in OOP a struct is like an object's data attributes
//A method in a struct is how you specify the behaviour that goes along with a struct

#[derive(Debug)] //We are adding the debug trait to our struct
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle { //impl -> implementation
    //Methods are similar to functions but they are defined with the context of a struct (or enum or a trait object)
    fn area(&self) -> u32 {
        //we dont want a mutable refernce becuase we only want to read data from the struct, not change it
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    //Associated functions are often used as constructors that will return a new instance of the struct.
    //Associated functions do not take `self` as a paramater
    fn square(size: u32) -> Rectangle {
        Rectangle {length: size, width: size}
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 20, width: 10 };
    let square = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    //Note! - Rust has automatic referencing and dereferncing
    //e.g. p1.distance(&p2) == (&p1).distance(&p2)


    //What if we want to print the struct?
    //unfortunately `println!(rect1)` will not work

    //However, we could print it using the debug identifier `{:?}` or `{:#?}` to print indented
    println!("rect1 is {:?}", rect1);
    println!("square is {:?}", square);
}
