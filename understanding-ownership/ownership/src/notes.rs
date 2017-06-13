fn variable_scope_heap_and_stack() {
    //Rust has a unique approach to memory management: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. No run-time costs are incurred for any of the ownership features.

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    //We’ve already seen string literals, where a string value is hardcoded into our program.
    let s = "world";
    //String literals are convenient, but they aren’t always suitable for every situation in which you want to use text. One reason is that they’re immutable.

    //Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, String.
    //This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    let s = String::from("hi there");

    //This kind of string can be mutated:
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    //Unlike other languages rust has no GC, but it also doesn't require explicit memory free call (like in C). Instead, once a heap allocated variable goes out of scope, it gets destroyed
}

fn move_fn() {
    // Say we have two variables like:
    let s1 = String::from("Hello");
    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap `Hello`
    let s2 = s1;
    //println!("{}", s1); - Not allowed!
    println!("{}", s2);

    //However, the above assignment poses a problem. When both s1 and s2 go out of scope, Rust will try to free the heap memory that both vairable pointers reference twice! This cannot happen
    //Rather than a shallow copy, this is known as a `move` in rust. s1 was moved to s2
}

fn clone_fn() {
    //However, if we do want create a copy in the heap we use the following:
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack

    //So what types are Copy? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can be Copy, and nothing that requires allocation or is some form of resource is Copy
}
