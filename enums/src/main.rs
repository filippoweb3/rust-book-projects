// enum IpAddrKind {
//    V4,
//    V6,
// }

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//enum Option<T> {
//    None,
//    Some(T),
//}

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

impl Message {
    fn call(&self) {

    }
}

fn main() {

//    let four = IpAddrKind::V4;
//    let six = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1")); // instance of IpAddr Type
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);

    let m = Message::Write(String::from("hello"));

    println!("{:#?}", m);

    m.call();

    let some_number = Some(5);
    let some_char = Some("e");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    // let sum = x + some_number; // not working as x and some_number are different Types!
    // need to handle the possibility that Option<i8> is not a valid value / null value

    let t = value_in_cents(Coin::Quarter(UsState::Alaska));

    dbg!(t);

    // println!("{t}");

}

// fn route(ip_kind: IpAddrKind) -> IpAddrKind {
//    ip_kind
// }

fn value_in_cents(coin: Coin) -> u8 { // takes an argument of the Type Coin
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        },
    }
}

fn 
