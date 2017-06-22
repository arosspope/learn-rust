//Generics are abstract stand-ins for concrete types or other properties. We can express properties of generics, such as their behavior or how they relate to other generics, without needing to know when we're writing and compiling the code what will actually be in their place.

//Generic struct
struct Point<T> {
    x: T,
    y: T,
}

//generic with two different types
struct DifferentPoint<T, U>{
    x: T,
    y: U
}

//Implementing generic method for generic struct
impl<T> Point<T> { //notice we have to declare `T` after `impl`, so that we can use it when we specify that we're implementing methods on the type Point<T>
    fn x(&self) -> &T {
        &self.x
    }
}

//consider this mixup method
impl<T, U> DifferentPoint<T, U> {
    //Notice that the method must define the type constansts used in the fn paramaters
    fn mixup<V, W>(self, other: DifferentPoint<V, W>) -> DifferentPoint<T, W> {
        DifferentPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    //We can use our are generic Point struct
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 1.0, y: 3.0};
    //let p3 = Point {x: 2, y: 4.5}; - Would not work as the types are dissimilar
    let p3 = DifferentPoint {x: 2, y: 4.5}; //Okay!
    let p4 = DifferentPoint {x: "spicy", y: 0}; //Okay!
    let p5 = p4.mixup(p3); //x: "spicy", y: 4.5

    let numbers = vec![1, 2, 5, 10];
    let chars = vec!['y', 'm', 'a', 'q'];

    println!("p1.x = {}", p1.x());

}

//Generic function finding the largest 'thing' from a list
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest { //This will not compile as we need to implement the PartialOrd trait
            largest = item;
        }
    }

    largest
}
