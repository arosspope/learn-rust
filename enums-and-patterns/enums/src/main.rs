// enum IpAddrKind {
//     V4, //The values of an enum are known as its variants
//     V6,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    //V7(some struct!) - a variant could have a struct type!
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Hello, world!");
}
