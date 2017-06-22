use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
//Rust doesn’t have exceptions. Instead, it has the value Result<T, E> for recoverable errors and the panic! macro that stops execution when it encounters unrecoverable errors.

//When this macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.
fn main() {
    recoverable_error();
    panic_examples();

}

fn panic_examples(){
    let v = vec![1, 2, 3];
    v[100];                             //Will panic
    panic!("crash and burn baby!");     //panic macro
}

fn recoverable_error(){
    //We can recover errors uing the `Result` type
    let f = File::open("hello.txt");

    //Simple error handling (error or its not)
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    //More complex, handling error types,
    let f2 = File::open("hello.txt");
    let f2 = match f2 {
        Ok(file) => file,
        //This if branch is known as a `match guard` (further check, before following the branch)
        //In the context of a pattern, & matches a reference and gives us its value, but ref matches a value and gives us a reference to it.
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    //Simple way
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); //Will panic if not open
    read_username_from_file();
}

//shortcut for propgating errors
fn read_username_from_file() -> Result<String, io::Error> {
    //The '?' operator allows us to...
    //If the value is an Err, the value inside the Err will be returned from the whole function as if we had used the return keyword so that the error value gets propagated to the caller.
    //? Can Only Be Used in Functions That Return Result (i.e. you couldn't use it main)
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
