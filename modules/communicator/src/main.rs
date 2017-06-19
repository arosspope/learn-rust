//We use the extern crate command to bring the communicator library crate into scope, because our package actually now contains two crates.
//Cargo treats src/main.rs as the root file of a binary crate, which is separate from the existing library crate whose root file is src/lib.rs.

//Also note that even if we're using an external crate within a submodule of our project,
//the extern crate should go in our root module (so in src/main.rs or src/lib.rs)
extern crate communicator;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow}; //Using the `use` keyword to make the code less complicated
//This also works `use TrafficLight::*`

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green; // because we didn’t `use` TrafficLight::Green

    communicator::client::connect();
}
