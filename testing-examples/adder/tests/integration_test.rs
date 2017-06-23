//Rust treats the tests directory specially, and will only compile files in this directory with `cargo test`
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
