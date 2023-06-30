use std::{fmt::Display, ops::Deref};

fn main() {
    let char_lower_a = 'a';
    let bytes_a = char_lower_a.to_string().into_bytes();
    let bytes_emoji = "👍🏻".to_string().into_bytes();
    println!("bytes_a: {bytes_a:?}");
    println!("bytes_emoji: {bytes_emoji:?}");

    let my_type = MyType('a');
    test(&my_type);

    let string = String::from("aaa");
    let chars = string.chars().collect::<Vec<char>>();
    println!("vec<char>: {:?}", chars);
    println!("slice: {:?}", chars.as_slice());

    let _tmp = chars[0];
    println!("_tmp: {_tmp}");

    let c: i8 = 127;
    let c_2: i8 = -1;
    println!("{:08b}", c);
    println!("{:08b}", c_2);

    let u_c_3: u8 = 254;
    let u_c: u8 = 127;
    let u_c_2: u8 = 1;
    println!("u_c_3: {:08b}", u_c_3);
    println!("u_c: {:08b}", u_c);
    println!("u_c_2: {:08b}", u_c_2);
}

struct MyType(char);

impl Deref for MyType {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for MyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyType({})", self.0)
    }
}

fn test(input: &char) {
    println!("I nedd a char to print: {input}");

    let my_type = MyType('a');
    println!("Character: {}", my_type.to_uppercase());
}
