
fn main() {
    //Iterators are a pattern in Rust that allows you to do some processing on a sequence of items.
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, [2, 3, 4]);

    let mut counter = Counter::new();
    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    let x = counter.next();
    println!("{:?}", x);

    //Note that zip produces only four pairs; the theoretical fifth pair (5, None) is never produced because zip returns None when either of its input iterators return None.
    let sum: u32 = Counter::new().take(5)
                             .zip(Counter::new().skip(1))
                             .map(|(a, b)| a * b)
                             .filter(|x| x % 3 == 0)
                             .sum();

    assert_eq!(18, sum);

}

//================ Implementing the Iterator trait =======
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // Our iterator will produce u32s
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
