fn main() {
    let characters = ['a', 'A', 'z', 'Z'];

    for character in characters {
        println!("{character}: {}", character as u32);
    }

    let i8_items: Vec<i8> = vec![-128, -127, -126, -125, -1, 1, 124, 125, 126, 127];

    for item in i8_items {
        println!("{item}: {:08b} {:?}", item, item.count_ones());
    }

    let u8_items: Vec<u8> = vec![1, 124, 125, 126, 127, 254, 255];

    for item in u8_items {
        println!("{item}: {:08b} {:?}", item, item.count_ones());
    }
}
