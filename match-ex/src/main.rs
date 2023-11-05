#[derive(Debug)]
enum JpPrefecture {
    Gifu,
    Osaka,
    Tokyo,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(JpPrefecture),
}

fn value_in_cents(coin: Coin) ->u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            35
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");
    let cent = value_in_cents(Coin::Quarter(JpPrefecture::Gifu));
    println!("cent is {}", cent);
    
    let five = Some(5);
    let six = plus_one(five);
    // println!("six is {}", six);
    let none = plus_one(None);
    // println!("node is {}", none);

}
