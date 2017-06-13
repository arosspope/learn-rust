fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    //In passing a reference the value s1 is not `moved` to another variable

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("Hi");
    change(&mut s2);
    println!("{}", s2); //Hi, world!

    multiple_refernces();
}

//This function expects a reference to a variable of type `String`
fn calculate_length(s: &String) -> usize {
    s.len()
    //s does not drop what is on the heap, as it does not have `ownerhsip` of the var
    //This is also known as `borrowing`

    //however, we are not allowed to modify a borrowed value (references are immutable by default)
    //s.push_str(", world"); - No!!
}

fn change(s: &mut String) {
    //We can now modify a string with a mutable reference
    s.push_str(", world!");

    //But mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope
}

fn  multiple_refernces(){
    let mut s = String::from("hello");
    let mut w = String::from("world");

    {
        let r1 = &mut s;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //We are not allowed to mix mutable and imutable references
    let r3 = &w; // no problem
    let r4 = &w; // no problem
    //let r5 = &mut w; // BIG PROBLEM
}

//These are not allowed, and are checked at compile time!
fn dangaling_references() {
    //let reference_to_nothing = dangle();
}

fn dangle(){ //-> &String {
    let s = String::from("hello");

    //&s
} //s goes out of scope here so the reference is pointing to garbage in memory
