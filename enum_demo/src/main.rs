
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
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
        Coin::Quarter(state) => {
            println!("quater state {state:?}");
            25
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
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));
    
    let five = Some(5);
    println!("function = {:?}", plus_one(five));
    println!("function none = {:?}", plus_one(None));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max number {max}");
    }

    let mut count = 0;
    let state = UsState::Alabama;
    let coin = Coin::Quarter(state);
    if let Coin::Quarter(state) = coin {
        println!("state coin {state:?}");
    } else {
        count += 1;
    }
    println!("count = {count}");
}
