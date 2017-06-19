pub mod client; //pulling in another module called client.rs and making it public
mod network;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        //In the tests module, we can either use leading colons to let Rust know that we want to start from the root and list the whole path
        ::client::connect();
        //or use `super`
        super::client::connect();

        client::connect(); //only works because of `use statement within module`
    }
}
