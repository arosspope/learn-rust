#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

struct Guess {
    value: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value: value,
        }
    }
}

#[cfg(test)] //Unit tests (for testing modules) -> different to integration tests!
mod tests {
    use super::*;

    #[test] //This is an attribute to annotate that this function is a test
    fn exploration() {
    }

    #[test]
    fn can_hold_test() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller), "smaller rectangler was detected as being larger");
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2)); //Rust allows testing of private functions!
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")] //Allows us to catch panics if we expect them
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        // run wth `cargo test -- -ignored`
    }
}
