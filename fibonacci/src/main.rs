use std::io;

fn main() {
    println!("Fibonacci sequence");
    println!("How many elements of the sequence do you wish to generate? ");
    let mut limit = String::new();

    io::stdin().read_line(&mut limit)
        .expect("Failed to read line");

    let limit: u32 = limit.trim().parse()
        .expect("That is not a valid number");

    let mut f_n_2 = 0;
    println!("fib[0]: {}", f_n_2);
    let mut f_n_1 = 1;
    println!("fib[1]: {}", f_n_1);

    for i in 2..limit {
        let f_i = f_n_1 + f_n_2;
        f_n_2 = f_n_1;
        f_n_1 = f_i;

        println!("fib[{}]: {}", i, f_i);
    }
}
