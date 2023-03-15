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

fn main() {
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
}
