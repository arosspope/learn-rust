//Rust gives you the ability to define closures, which are similar to functions
fn main() {
    add_one_example();
    calculate_example();
    closure_enviornment_example();

    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);
}

fn add_one_example() {
    //The closure definition, on the first line, shows that the closure takes one parameter named x. Parameters to closures go in between vertical pipes (|).
    let add_one = |x| x + 1;
    let add_one_v2 = |x: i32| -> i32 { x + 1 }; //A more verbose way to write a closure
    //usually though, Rust is able to infer parameter types etc

    let five = add_one(4);

    assert_eq!(5, five);
    assert_eq!(6, add_one_v2(5));
}

fn calculate_example() {
    let calculate = |a, b| {
        let mut result = a * 2;

        result += b;

        result
    };

    assert_eq!(7, calculate(2, 3)); // 2 * 2 + 3 == 7
    assert_eq!(13, calculate(4, 5)); // 4 * 2 + 5 == 13
}

fn closure_enviornment_example() {
    //We've learned that functions can only use variables that are in scope, either by being const or being declared as parameters.
    //Closures can do more: they're allowed to access variables from their enclosing scope.
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

//Closures as Function Parameters Using the Fn Traits
fn call_with_one<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {
        //The some_closure parameter has the generic type F, which in the where clause is defined as having the trait bounds Fn(i32) -> i32.
        //The Fn trait represents a closure, and we can add types to the Fn trait to represent a specific type of closure.
        //Fn isn't the only trait bound available for specifying closures, however. There are three: Fn, FnMut, and FnOnce
    some_closure(1)
}
