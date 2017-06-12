fn main() {
    simple_variables();
    constants();
    shadowing();
}

fn simple_variables() {
    let x = 5;
    //x = 6 - Not allowed! As x is immutable
    println!("The value of x is: {}", x);

    let mut y = 7;
    println!("The value of y is: {}", y);
    y = 8; //This is allowed!
    println!("The value of y is: {}", y);
    println!();
}

fn constants() {
    //Like immutable variables, constants are also values that are bound to a name and are not allowed to change.. but there are differences!

    //Type must be specified on a constant
    const FOO: u32 = 100_000;
    println!("The value of FOO is {}", FOO);
    println!();
}

fn shadowing() {
    let x = 5;
    let x = x + 1; //Notice how we are allowed to re-assign an existing immutable variable by using the let keyword again - this is known as `shadowing` a variable
    println!("The value of x is: {}", x);

    //With shadowing, we are allowed to change the type of the variable like so...
    let x = "this is now a string";
    println!("The value of x is: {}", x);
    println!();
}
