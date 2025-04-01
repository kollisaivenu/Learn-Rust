enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// enum Option<T>{
//     Node,
//     Some(T),
// }
fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::1"),
    };
    let m = Message::Write(String::from("hello"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max is {}", max);
    }

}

fn learn_match(){
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => {
            println!("State quarter from {state:?}}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

