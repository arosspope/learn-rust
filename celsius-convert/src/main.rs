use std::io;

fn main() {
    //Some samples using what we learnt
    println!("°C to °F convertor");
    println!("Please enter temperature in celsius: ");
    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = temp.trim().parse()
        .expect("That is not a valid number");

    println!("{}°C = {}°F", temp, to_fahrenheit(temp));
}

fn to_fahrenheit(temp_celsius: f64) -> f64 {
    (temp_celsius * 1.8) + 32.0
}
