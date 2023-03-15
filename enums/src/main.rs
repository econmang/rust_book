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
}
