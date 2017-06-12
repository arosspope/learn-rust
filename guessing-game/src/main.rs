extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("-- Guess the number game --\n"); //simple print to stdout

    //`thread_rng` generates a random number local to the current thread of execution
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //the `loop` keyword gives us an infinite loop
    loop {
        println!("Please  input your guess...");

        //`String::new()` returns a new instance of a String
        //`::new` indicates that `new` is an assoicated function of the string type
        //an associated function is implemented on a type, rather than on an instance (static method)
        //by default, all variables are immutable (cannot be re-assigned)
        let mut guess = String::new();

        //the `&mut guess` passes a reference to a mutable variables
        //like variables, references are immutable by default and so the `&mut` keyword must be supplied
        //read_line returns an enum Err or OK. `.expect` will trigger when an Err is returned
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //we should convert our guess to a number so it can be compared to the secret number
        //we are shadowing the previous value of guess with a new one. shadowing lets us reuse the guess variable (and even change its type!)
        //specifying `u32` is essential as parse can return a variety of types
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            //skip rest of processing and start loop again
            //`_` is a catchall value
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        //the `cmp` function returns a variant of the Ordering enum.
        //the `match` expression to decide what to do next based on which variant was returned
        //a `match` expression is made up of arms. An arm consits of a pattern and the code that should be run if that pattern is matched
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}
