fn main() {
    //Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    simple_slice();

    let phrase = String::from("Hello world");
    let first = first_word(&phrase);
    println!("First word: {}", first);
}

fn simple_slice() {
    let s = String::from("hello world");

    let hello = &s[..5]; //starting index is implictly 0

    //the slice data structure actually stores the starting position and the length of the slice.
    //So in the case of `world`, it would be a slice that contains a pointer to the 6th byte of s and a length value of 5.
    let world = &s[6..11];

    println!("{} {}", hello, world);
}

//if we changed the below signature to
//fn first_word(s: &str) -> &str {
//it would allow us to use this function for both types
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //Return the slice up to the index we are at
        }
    }

    &s[..] //Return the whole phrase
}
