fn main() {
    //if-else
    compare_to_five(1);
    compare_to_five(6);
    compare_to_five(8);

    //assigning variables using if
    fancy_let(0);
    fancy_let(5);

    //loops
    infinite_loop();
    while_loop();
    for_loop();
}

fn compare_to_five(number: i32){
    //Simple if else statement
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn fancy_let(number: i32) {
    let x: &str = if number == 0 {
        "zero"
    } else {
        "non zero"
    };

    println!("x is {}", x);
}

fn infinite_loop() {
    let mut x = 0;
    loop {
        if x == 5 {
            break;
        }

        println!("infinite looping");
        x = x + 1;
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //`.rev()` reverses the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
