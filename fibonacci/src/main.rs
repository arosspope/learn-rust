use std::env;

fn main() {
    let args: Vec <_> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: ./fibonacci <number of elements>");
    }

    println!("~~~ Fibonacci Generator ~~~");
    let limit: u64 = args[1].trim().parse()
        .expect("That is not a valid unsigned number");

    println!();

    for i in 0..limit {
        print!(" {}", fibonacci(i));
    }

    println!();
}


fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
