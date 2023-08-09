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

fn main() {
    let characters = ['a', 'A', 'z', 'Z'];

    for character in characters {
        println!("{character}: {:?}", character as u8);
    }

    let i8_items: Vec<i8> = vec![-128, -127, -126, -125, -1, 1, 124, 125, 126, 127];

    for item in i8_items {
        println!("{item}: {:08b} {:?}", item, item.count_ones());
    }

    let u8_items: Vec<u8> = vec![1, 124, 125, 126, 127, 254, 255];

    for item in u8_items {
        println!("{item}: {:08b} {:?}", item, item.count_ones());
    }

    let _range_1: std::ops::Range<char> = 'a'..'z';

    let a = UsState::Alabama;
    let coin = Coin::Quarter(a);

    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => println!("Not a state quarter"),
    }
}
