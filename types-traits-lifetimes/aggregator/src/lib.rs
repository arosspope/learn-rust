//Traits allow us to abstract over behavior that types can have in common
//Traits are often called 'interfaces' in other languages (with some differences)

//The behavior of a type consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together in order to define a set of behaviors necessary to accomplish some purpose.

//Media aggregator library

pub trait Summarizable {
    //Each type that implements this trait must provide its own custom behavior
    //Define a trait with default behavior
    fn summary(&self) -> String {
        String::from("(Read more ...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle { //Implementing a trait
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {} //Will use default beahvior


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
