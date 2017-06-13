fn main() {
    basic_types();
    compound_types();
    arrays();
}

fn arrays() {
    //Arrays are useful when you want your data allocated on the stack rather than the heap
    //arrays are similar to vectors (list), but that is explained in the vectors project
    let a = [1,2,3,4];
    let first = a[0];

    for element in a.iter() {
        println!("The value is {}", element);
    }

    println!();
}

fn compound_types() {
    let tup: (i32, f64, char) = (500, 6.4, '👌');
    println!("The value of tup[0] is: {}", tup.1);

    //The variable tup binds to the entire tuple, since a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

    let (x, y, z) = tup;
    println!("The value of z is: {}", z);
    println!();
}


fn basic_types(){
    //Rust is a statically typed language which means it must know the types of all variables at compile time

    // Scalar types represent a single value. Rust has four primary types:
    //     - integer
    //     - boolean
    //     - floating-point
    //     - chars

    // Integers length/types include:
    //     - 8bit: i8, u8
    //     - 16bit: i16, u16
    //     - 32bit: ...
    //     - 64bit: ...
    //     - arch: isize, usize


    //All number literals (expect the byte literal) allow a type suffix:
    let x = 57u8;
    println!("The value of x is: {}", x);

    //rust can use visual sepeartors "_"
    let y = 0b1111_0000;
    println!("The value of y is: {}", y);

    //Integer types default to i32
    //The primary situation in which you’d use isize or usize is when indexing some sort of collection.

    //Floating point (default is f64)
    let x = 2.0;        //f64
    let y: f32 = 3.0;   //f32

    //Booleans
    let x = true;
    let y: bool = false;

    //Characters
    let c = 'z';
    let face: char = '😃'; //supports Unicode
    println!("The value of face is: {}", face);
    println!();
}
