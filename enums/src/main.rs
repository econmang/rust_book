/*
* Original implementation splits enum and struct
enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => {println!("V4 address passed");}
        IpAddrKind::V6 => {println!("V6 address passed");}
    }
}
*/

enum IpAddr {
    V4(String),
    V6(String),
}

// Unlike structs, each variant of an enum can have different types associated
enum IpAddrDiffTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

// In order to represent these as structs, you'd have to generate a QuitMessage, MoveMessage, etc.
// instead of leveraging variants in an enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Similarly to structs, you have to utilize the impl keyword to create methods on them
impl Message {
    fn call(&self) {
        match &self {
            Message::Quit => { println!("QUITTER"); },
            _ => {},
        }
    }
}

fn main() {
    /*
    * Represents the original implmentation above
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let home_type: String;
    let lb_type: String;
    match home.kind {
        IpAddrKind::V4 => {home_type = String::from("V4")}
        IpAddrKind::V6 => {home_type = String::from("V6")}
    };
    match loopback.kind {
        IpAddrKind::V4 => {lb_type = String::from("V4")}
        IpAddrKind::V6 => {lb_type = String::from("V6")}
    };
    println!("Home: {}, {}", home.address, home_type);
    println!("Loopback {}, {}", loopback.address, lb_type);
    */

    // Enum variants with directly identified data
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    let _home_diff = IpAddrDiffTypes::V4(127,0,0,1);
    let _loopback_diff = IpAddrDiffTypes::V6(String::from("::1"));
    println!("Needing to specify the type of IP address utilized is so common, it exists in the standard library...\nSee std::net::IpAddr for its definition.");

    let quit = Message::Quit;
    let _movement = Message::Move { x: 3, y: 2};
    let _hello = Message::Write(String::from("HELLO WORLD"));
    let _color = Message::ChangeColor(32, 32, 147);
    quit.call();
}
