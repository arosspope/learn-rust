fn main() {
    simple_fn();
    parameter_fn(5, 6.3);
    statements_and_expressions();

    let five = plus_one(4);
    println!("The value of return function is: {}", five);
}

fn simple_fn(){
    //Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words
    println!("Hello, simple function!");
    println!();
}

fn parameter_fn(x: i32, y: f32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!();
}

fn statements_and_expressions() {
    //Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value
    let y = 6; //overall a statement, `6` is an expression
    println!("The value of y is: {}", y); //calling a macro/function is an expression

    let y = {
        let x = 3;
        x + 1   //This is an expression
        //Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value
    };
    println!("The value of y is: {}", y);

    println!();
}

//Function with return value
fn plus_one(x: i32) -> i32 {
    x + 1 //last returning statement is an expression, no semicolon!
}
