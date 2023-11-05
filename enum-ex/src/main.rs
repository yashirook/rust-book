enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IpAddrKind) {
    // println!(ip_type)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call function is called")
    }
}

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // T と Option<T>は加算不可
    // let sum = x+y;
}
