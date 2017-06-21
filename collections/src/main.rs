use std::collections::HashMap;

fn main() {
    vector_example();
    string_example();
    hashmap_example();
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_example(){
    //Vectors allow us to store more than one value in a single data structure
    //Vectors can only store values of the same type

    //creating
    let v: Vec<i32> = Vec::new();

    //Rust can infer the type of value so the above is rarely needed
    let z = vec![1, 2, 3];

    //To create a vector and add new elements we `push`
    let mut y = Vec::new();
    y.push(3);
    y.push(10);

    //like any other struct, a vector will be freed when it goes out of scope

    //Reading elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2); //this will return None if out of bounds

    //Using variants of an enum to store different types within a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn string_example(){
    //The String & str types have mostly been covered, but here is some other neat stuff:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn hashmap_example(){
    //Creating a HashMap<K, V> type
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    //Fancier way to init hashmap using iterables and Vectors
    //`zip` method creates a vector of tuples where "Blue" is paired with 10 etc
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures

    //Types that implement the `Copy` trait (like i32), will have values copied into hash map.
    //For owners like String, te values will be moved and the hashmap will own these values
    let field_name = String::from("Favorite color"); //moved to hashmap
    let field_value = String::from("Blue"); //same

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }

    //updating hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}
