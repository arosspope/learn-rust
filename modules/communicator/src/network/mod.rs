//A new module named network

//Everything inside this file is inside the namespace network.
//In this case, we have a single function, connect.
//If we wanted to call this function we would, network::connect()
fn connect(){
}

// sub-modules are also allowed
// mod server {
//     called like network::server::connect()
//     fn connect(){
//     }
// }

//However we want to extract server into its own file
mod server;
