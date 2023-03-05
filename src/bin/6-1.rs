// 将数据附加到枚举的每个变体中
// 优点：
// 1. 可以将不同类型的数据附加到枚举的每个变体中
// 2. 可以将多个不同类型的数据附加到枚举的每个变体中
// 3. 不需要额外的 struct 来存储不同类型的数据
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
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

    let home1 = IpAddrKind::V4(127, 0, 0, 1);
    let loopback1 = IpAddrKind::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
