fn main() {
    ip_example();
    message_example();
    option_example();
}

//----------------------------- Simple enum ------------------------------------------
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_example() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
}

//----------------------------- Enum with methods ------------------------------------
enum Message {
    Quit,
    Move {x: i32, y: i32}, //This is an anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

//Defining methods on an enum
impl Message {
    fn call(&self) {
        //.. do something
    }
}

fn message_example() {
    let m = Message::Write(String::from("home"));
    let quit = Message::Quit;
    let mMove = Message::Move {
        x: 1,
        y: 2,
    };

    m.call();
}

//----------------------------- The option enum ------------------------------------------
//Rust does not have the concept of `null` instead they use `Option`
fn option_example() {
    //using the none type and its variants to init variables
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; //if we use `None` instead, we must explictly tell the compiler what type it is

    // Say we had:
    //      let x: i8 = 5;
    //      let y: Option<i8> = None;
    //      let sum = x + y;
    // This is not allowed as rust considers these variables two different types

    //unwrapping an option
    println!("some string = {}", some_string.unwrap());
    println!("an absent_number could also be {}", absent_number.unwrap_or(3));
}
