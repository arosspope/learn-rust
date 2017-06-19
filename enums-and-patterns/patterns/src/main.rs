//Rust has an extremely powerful control-flow operator called `match` that allows us to compare a
//value against a series of patterns and then execute code based on which pattern matches.

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);

    println!("The value of your coin is {}", Coin::value_in_cents(coin));

    match_with_option_example();
    what_to_do(1);
    what_to_do(3);
    if_let_example();
}

//----------------- Coin example ----------------------//
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(coin: Coin) -> i32 {
        match coin {
            Coin::Penny => {
                println!("You have a Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }
}

//----------------- Matching with Option<T> ----------------------//

fn match_with_option_example() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("value of five is {}", five.unwrap());
    println!("value of five + 1 is {}", six.unwrap());
    //println!("value of none is {}", none.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_to_do(x: u32) {
    match x {
        1 => println!("Hello"),
        2 => println!("Goodbye"),
        _ => println!("Who are you??"), //specifying () here means to do nothing
    }
}

//----------------- Concise Control Flow with `if let` ----------------------//

fn if_let_example() {
    //The `if let` syntax lets you combine if and let into a less verbose way to handle values that match one pattern and ignore the rest

    //This...
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    //Can be re-written as..
    if let Some(3) = some_u8_value {
        println!("three again");
    }

    //Choosing between match and if let depends on what you’re doing in your particular situation and if gaining conciseness is an appropriate trade-off for losing exhaustive checking.
}
