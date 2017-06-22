extern crate aggregator;
use aggregator::Summarizable;

fn main() {
    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let forecast = WeatherForecast {
        high_temp: 64.0,
        low_temp: 12.0,
        chance_of_precipitation: 100.0,
    };

    println!("1 new tweet: {}", tweet.summary());
    println!("Forecast: {}", forecast.summary());
    notify(tweet);
}

//Implementing a trait outside of its scope
struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

//One restriction to note with trait implementations: we may implement a trait on a type as long as either the trait or the type are local to our crate. In other words, we aren't allowed to implement external traits on external types
impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}

//=========================== Trait bounds ==================================
//We can constrain generic types so that rather than being any type, the compiler will ensure that the type will be limited to those types that implement a particular trait and thus have the behavior that we need the types to have

//Trait bounds on a generic type
pub fn notify<T: Summarizable>(item: T){
    println!("Breaking news! {}", item.summary());
}

//We can specify multiple trait bounds on a generic type by using `+` and `where`
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Summarizable + Clone, U: Clone {
    3
}
