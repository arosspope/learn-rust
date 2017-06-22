//Generics are abstract stand-ins for concrete types or other properties. We can express properties of generics, such as their behavior or how they relate to other generics, without needing to know when we're writing and compiling the code what will actually be in their place.

//Generic struct
struct Point<T> {
    x: T,
    y: T,
}

//Implementing generic method for generic struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    //We can use our are generic Point struct
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 1.0, y: 3.0};
    //let p3 = Point {x: 2, y: 4.5}; - Would not work as the types are dissimilar
}

//Generic function finding the largest 'thing' from a list
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        //Implement code to find largest thing
    }

    largest
}

//Without a generics, it would be pretty frustrating to find the largest 'thing'
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
