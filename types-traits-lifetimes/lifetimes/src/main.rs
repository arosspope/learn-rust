//Every reference in Rust has a `lifetime`, which is the scope for which that reference is valid
//Most of the time lifetimes are implicit and inferred, just like most of the time types are inferred
//There are cases where the lifetimes of references could be related in a few different ways, so Rust needs us to annotate the relationships using generic lifetime parameters
//The main aim of lifetimes is to prevent dangling references, which will cause a program to reference data other than the data we're intending to reference.
use std::fmt::Display;

//Lets write a program that returns the longest of two string slices
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); //will print abcd
    struct_example();
}

//return type needs a generic lifetime parameter on it!
//Rust can't tell if the reference being returned refers to s1 or s2
/*
fn longest_fail(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
*/

//What lifetime annotations do is relate the lifetimes of multiple references to each other.
//Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe '.
//The names of lifetime parameters are usually all lowercase, and like generic types, their names are usually very short. 'a is the name most people use as a default
/*
    Examples:
        &i32        // a reference
        &'a i32     // a reference with an explicit lifetime
        &'a mut i32 // a mutable reference with an explicit lifetime

    Just like generic type parameters, generic lifetime parameters need to be declared within angle brackets
*/

//The longest function definition that specifies all the references in the signature must have the same lifetime, 'a
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

//The function signature now says that for some lifetime 'a, the function will get two parameters, both of which are string slices that live at least as long as the lifetime 'a.
//The function will return a string slice that also will last at least as long as the lifetime 'a.
//This is the contract we are telling Rust we want it to enforce.

/* Consider the following example using the longest method:

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    } -> string2 goes out of scope

    println!("The longest string is {}", result); -> result could point to either string1 or 2!

    * The above code results in compile error due to the differing lifetimes of string1 and 2
    * the function longest detects this and throws the error

*/

//================ Lifetime annotations in struct definitions =============
//It is possible for structs to hold references, but we need to add a lifetime annotation on every reference in the struct's definition.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_example() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

//================ Static lifetimes =============
// The 'static lifetime is the entire duration of the program. All string literals have the 'static lifetime, which we can choose to annotate as follows:
//let s: &'static str = "I have a static lifetime.";

//====== Generic Type Parameters, Trait Bounds, and Lifetimes Together =====

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
